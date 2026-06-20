<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 font-sans p-6">
    <header class="mb-8 border-b border-slate-800 pb-6 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
      <div>
        <h1 class="text-2xl font-black tracking-wider text-emerald-400">AGRISENTRY ENTERPRISE</h1>
        <p class="text-sm text-slate-400">Centro de Operações Agrícolas (COA) • Live Telemetry Quality Monitor</p>
      </div>
      <div class="flex items-center gap-4">
        <div class="bg-slate-900 border border-slate-800 px-4 py-2 rounded-lg text-right">
          <span class="text-xs block text-slate-500 uppercase tracking-widest font-bold">Field Health Index</span>
          <span class="text-lg font-mono font-bold text-emerald-400">{{ fieldHealth }}%</span>
        </div>
        <button @click="triggerOtaScan" :disabled="scanning" class="bg-emerald-600 hover:bg-emerald-500 disabled:bg-slate-800 text-white font-bold text-xs uppercase tracking-wider px-5 py-3 rounded-lg transition-all duration-200 shadow-lg shadow-emerald-900/20">
          {{ scanning ? 'Scanning Network...' : '📡 Force OTA Scan' }}
        </button>
      </div>
    </header>

    <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
      <div v-for="(metric, idx) in metricsMap" :key="idx" class="bg-slate-900 border border-slate-800 p-4 rounded-xl">
        <span class="text-xs font-bold text-slate-500 uppercase tracking-wider">{{ metric.label }}</span>
        <div class="text-2xl font-mono font-bold mt-1" :class="metric.colorClass">{{ metric.count }}</div>
      </div>
    </section>

    <main class="grid grid-cols-1 lg:grid-cols-12 gap-6">
      <section class="lg:col-span-5 bg-slate-900 border border-slate-800 p-6 rounded-2xl flex flex-col justify-between">
        <div>
          <h2 class="text-md font-bold tracking-wide uppercase text-slate-300 mb-4">Simulate Field Telemetry Target</h2>
          
          <div class="mb-4">
            <label class="block text-xs uppercase font-bold text-slate-400 mb-2">Target Telemetry Node</label>
            <select v-model="form.device_id" class="w-full bg-slate-950 border border-slate-800 rounded-lg p-3 text-sm font-mono text-slate-200 focus:outline-none focus:border-emerald-500">
              <option value="ESP32-TEST-001">ESP32-TEST-001 (Sensor de Umidade Alpha)</option>
              <option value="esp32-gate-e50080">esp32-gate-e50080 (Temperatura/Umidade)</option>
              <option value="00:11:22:33:44:55">00:11:22:33:44:55 (Sensor de Teste Python)</option>
              <option value="esp32-gate-7a8d33">esp32-gate-7a8d33 (Monitor Irrigação)</option>
              <option value="esp32-gate-275e00">esp32-gate-275e00 (Sensor Estufa)</option>
              <option value="esp32-gate-5b12c9">esp32-gate-5b12c9 (Sensor Solo/Luz)</option>
              <option value="esp32-gate-8b3308">esp32-gate-8b3308 (Estação Meteorológica)</option>
            </select>
          </div>

          <div class="mb-6">
            <label class="block text-xs uppercase font-bold text-slate-400 mb-2">Manual Signal Matrix Value</label>
            <input type="number" step="0.1" v-model.number="form.reading_value" class="w-full bg-slate-950 border border-slate-800 rounded-lg p-3 font-mono text-slate-200 text-lg focus:outline-none focus:border-emerald-500" />
          </div>

          <div class="grid grid-cols-3 gap-2 mb-6">
            <button @click="applyTemplate(45.2)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2 rounded-lg font-medium transition">Normal Data</button>
            <button @click="applyTemplate(0.0)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2 rounded-lg font-medium transition">Sensor Noise</button>
            <button @click="applyTemplate(145.8)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2 rounded-lg font-medium transition">Critical Outlier</button>
          </div>
        </div>

        <button @click="dispatchTelemetry" class="w-full bg-slate-100 hover:bg-white text-slate-950 font-bold text-xs uppercase tracking-widest py-4 rounded-xl transition shadow-xl">
          Transmit Payload Matrix
        </button>
      </section>

      <section class="lg:col-span-7 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col h-[420px]">
        <div class="border-b border-slate-800 px-4 py-3 flex justify-between items-center bg-slate-900/50 rounded-t-2xl">
          <span class="text-xs font-bold uppercase tracking-wider text-slate-400 font-mono">Live Infrastructure Logging Terminal</span>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
            <span class="text-[10px] font-mono tracking-widest text-emerald-500 uppercase font-bold">Streaming Active</span>
          </div>
        </div>
        <div ref="terminal" class="p-4 flex-1 overflow-y-auto font-mono text-xs space-y-2 bg-slate-950 rounded-b-2xl scrollbar-thin">
          <div v-for="(log, idx) in logs" :key="idx" class="leading-relaxed whitespace-pre-wrap">
            <span class="text-slate-500">[{{ formatTime(log.created_at) }}]</span>
            <span class="font-bold ml-2 px-1 rounded text-[10px]" :class="getComponentColor(log.component)">{{ log.component }}</span>
            <span class="ml-2" :class="getLevelColor(log.level, log.message)">{{ log.message }}</span>
          </div>
          <div v-if="logs.length === 0" class="text-slate-600 text-center py-12 italic">Awaiting telemetry frames to open streaming data...</div>
        </div>
      </section>
    </main>
  </div>
</template>

<script>
export default {
  data() {
    return {
      // Direct production URL targeting Render backend to bypass local Vite caching issues
      apiUrl: 'https://agrisentry-iot-gateway.onrender.com', 
      scanning: false,
      form: { 
        device_id: 'ESP32-TEST-001', 
        reading_value: 45.2 
      },
      metrics: [],
      logs: []
    }
  },
  computed: {
    // Maps API raw statistics data onto structured UI display metadata with sanitization
    metricsMap() {
      const findCount = (status) => {
        const target = status.toUpperCase().replace('_', '');
        return this.metrics.find(m => {
          const current = m.status.toUpperCase().replace('_', '');
          return current === target || current.includes(target) || target.includes(current);
        })?.count || 0;
      };

      return [
        { label: 'Pending Evaluation', count: findCount('PENDING'), colorClass: 'text-amber-400' },
        { label: 'Validated Records', count: findCount('VALID'), colorClass: 'text-emerald-400' },
        { label: 'Signal Interference', count: findCount('ANOMALY_NOISE'), colorClass: 'text-sky-400' },
        { label: 'Critical Outliers', count: findCount('ANOMALY_CRITICAL'), colorClass: 'text-rose-500' }
      ];
    },
    // Computes overall operation stability percentage using standardized key lookup
    fieldHealth() {
      const findCount = (status) => {
        const target = status.toUpperCase().replace('_', '');
        return this.metrics.find(m => m.status.toUpperCase().replace('_', '').includes(target))?.count || 0;
      };

      const valid = findCount('VALID');
      const noise = findCount('ANOMALY_NOISE');
      const critical = findCount('ANOMALY_CRITICAL');
      const total = valid + noise + critical;
      return total ? Math.round((valid / total) * 100) : 100;
    }
  },
  mounted() {
    // Initialize polling process on lifecycle mount
    this.pollEngine();
    setInterval(this.pollEngine, 2500);
  },
  methods: {
    // Polls metrics and telemetry log events from the backend architecture
    async pollEngine() {
      try {
        const activeUrl = this.apiUrl; // Strictly targets the configured Render production endpoint

        // Fetch aggregation metrics for the main upper dashboard view
        const mRes = await fetch(`${activeUrl}/api/v1/dashboard/metrics`);
        if (mRes.ok) {
          const data = await mRes.json();
          this.metrics = data.metrics || [];
        } else {
          console.warn(`Metrics fetch failed with status: ${mRes.status}`);
        }

        // Fetch system pipeline telemetry logs
        const lRes = await fetch(`${activeUrl}/api/v1/dashboard/logs`);
        if (lRes.ok) {
          this.logs = (await lRes.json()).reverse();
          this.$nextTick(this.scrollToBottom);
        } else {
          console.warn(`Logs fetch failed with status: ${lRes.status}`);
        }
      } catch (e) {
        console.error("Error communicating with the Rust Gateway:", e);
      }
    },
    // Submits simulated JSON telemetry payloads down into the processing pipeline
    async dispatchTelemetry() {
      try {
        const activeUrl = this.apiUrl;

        const payload = {
          device_id: this.form.device_id,
          sensor_type: "SIMULATED_SENSOR",
          reading_value: parseFloat(this.form.reading_value),
          timestamp: new Date().toISOString()
        };

        const response = await fetch(`${activeUrl}/api/v1/telemetry`, {
          method: 'POST',
          headers: { 
            'Content-Type': 'application/json' 
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error(`HTTP Error Status: ${response.status}`);
        }

        // Refresh tracking values immediately following dispatch success
        this.pollEngine();
      } catch (e) {
        console.error("Error transmitting telemetry data:", e);
      }
    },
    // Simulates an Over-The-Air hardware device scan process
    triggerOtaScan() {
      this.scanning = true;
      setTimeout(() => {
        this.scanning = false;
        this.pollEngine();
      }, 1200);
    },
    // Preset injector for testing edge cases within simulation telemetry form
    applyTemplate(val) {
      this.form.reading_value = val;
    },
    // Manages scrolling element bounds inside the terminal mock wrapper
    scrollToBottom() {
      const term = this.$refs.terminal;
      if (term) term.scrollTop = term.scrollHeight;
    },
    // Strips full ISO strings to simple local timestamp representations
    formatTime(str) {
      if (!str) return '';
      return new Date(str).toLocaleTimeString();
    },
    // Dynamically manages CSS styles based on microservice origin tags
    getComponentColor(comp) {
      if (!comp) return 'bg-slate-950 text-slate-400';
      if (comp.includes('RUST')) return 'bg-orange-950 text-orange-400 border border-orange-800';
      if (comp.includes('AI')) return 'bg-purple-950 text-purple-400 border border-purple-800';
      return 'bg-blue-950 text-blue-400 border border-blue-800';
    },
    // Determines dynamic colors matching standard infrastructure log urgency ranks
    getLevelColor(level, msg) {
      if (!msg) return 'text-slate-300';
      if (msg.includes('ANOMALY_CRITICAL') || msg.includes('Outlier')) return 'text-rose-400 font-bold';
      if (msg.includes('ANOMALY_NOISE') || msg.includes('drop')) return 'text-sky-400';
      if (level === 'WARN') return 'text-amber-400';
      return 'text-slate-300';
    }
  }
}
</script>

<style scoped>
/* Customs scrollbar formatting styling parameters */
.scrollbar-thin::-webkit-scrollbar {
  width: 4px;
}
.scrollbar-thin::-webkit-scrollbar-track {
  background: transparent;
}
.scrollbar-thin::-webkit-scrollbar-thumb {
  background: #334155;
  border-radius: 2px;
}
.scrollbar-thin::-webkit-scrollbar-thumb:hover {
  background: #475569;
}
</style>
