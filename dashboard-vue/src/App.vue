<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 font-sans p-4 md:p-8 selection:bg-emerald-500/30">
    
    <header class="mb-8 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 bg-slate-900 border border-slate-800 p-6 rounded-3xl shadow-2xl">
      <div class="flex items-center gap-4">
        <div class="bg-emerald-500/10 p-4 rounded-2xl border border-emerald-500/20">
          <svg class="w-8 h-8 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path>
          </svg>
        </div>
        <div>
          <h1 class="text-3xl font-black tracking-tight text-white">AgriSentry<span class="text-emerald-400">.Enterprise</span></h1>
          <p class="text-sm text-slate-400 font-medium mt-1">Central de Operações Agrícolas (COA) • Monitoramento IA</p>
        </div>
      </div>

      <div class="flex flex-col sm:flex-row items-center gap-4 w-full md:w-auto">
        <div class="flex items-center gap-4 bg-slate-950 border border-slate-800 px-5 py-3 rounded-2xl w-full sm:w-auto">
          <div>
            <span class="text-[10px] block text-slate-500 uppercase tracking-widest font-bold">Field Health Index</span>
            <div class="flex items-baseline gap-1">
              <span class="text-2xl font-black text-emerald-400">{{ fieldHealth }}</span>
              <span class="text-sm font-bold text-emerald-600">%</span>
            </div>
          </div>
          <svg class="w-10 h-10 text-emerald-500/20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
          </svg>
        </div>
        
        <button @click="triggerOtaScan" :disabled="scanning" class="w-full sm:w-auto flex items-center justify-center gap-2 bg-gradient-to-r from-emerald-600 to-emerald-500 hover:from-emerald-500 hover:to-emerald-400 disabled:from-slate-800 disabled:to-slate-800 disabled:text-slate-500 text-white font-bold text-xs uppercase tracking-wider px-6 py-4 rounded-2xl transition-all duration-300 shadow-lg shadow-emerald-900/20 active:scale-95 border border-emerald-400/20">
          <svg v-if="!scanning" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"></path>
          </svg>
          <svg v-else class="w-5 h-5 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
          {{ scanning ? 'Scanning Network...' : 'Force OTA Scan' }}
        </button>
      </div>
    </header>

    <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <div v-for="(metric, idx) in metricsMap" :key="idx" class="bg-slate-900 border border-slate-800 p-5 rounded-3xl flex items-center gap-4 hover:border-slate-700 transition-colors shadow-xl shadow-black/20">
        <div class="p-3 rounded-2xl border" :class="metric.iconBgClass">
          <div v-vhtml="metric.svg" class="w-6 h-6" :class="metric.colorClass"></div>
        </div>
        <div>
          <span class="text-[10px] font-bold text-slate-500 uppercase tracking-widest">{{ metric.label }}</span>
          <div class="text-3xl font-black tracking-tight text-white mt-0.5">{{ metric.count }}</div>
        </div>
      </div>
    </section>

    <section class="mb-8">
      <div class="flex items-center justify-between mb-4 px-2">
        <h2 class="text-sm font-bold tracking-widest uppercase text-slate-400">Live Field Nodes</h2>
        <span class="text-xs font-mono text-emerald-500 bg-emerald-500/10 px-3 py-1 rounded-lg border border-emerald-500/20 shadow-inner">
          Active Nodes: {{ sensors.length }}
        </span>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        <div v-for="sensor in sensors" :key="sensor.id" class="bg-slate-900 border border-slate-800 p-5 rounded-3xl relative overflow-hidden group hover:border-slate-700 transition-all shadow-xl shadow-black/20">
          
          <div class="absolute -top-10 -right-10 w-32 h-32 blur-3xl opacity-10 transition-all group-hover:opacity-20"
               :class="sensor.status === 'VALID' ? 'bg-emerald-500' : 'bg-rose-500'"></div>

          <div class="flex justify-between items-start mb-4 relative z-10">
            <div class="flex items-center gap-3">
              <div class="p-2.5 rounded-xl bg-slate-950 border border-slate-800 text-slate-400" v-html="getSensorIcon(sensor.type)"></div>
              <div>
                <h3 class="text-sm font-bold text-slate-200">{{ sensor.name }}</h3>
                <p class="text-[10px] text-slate-500 font-mono mt-0.5">{{ sensor.id }}</p>
              </div>
            </div>
            <div class="flex h-3 w-3 relative mt-1">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full opacity-75"
                    :class="sensor.status === 'VALID' ? 'bg-emerald-400' : 'bg-rose-400'"></span>
              <span class="relative inline-flex rounded-full h-3 w-3"
                    :class="sensor.status === 'VALID' ? 'bg-emerald-500' : 'bg-rose-500'"></span>
            </div>
          </div>

          <div class="mb-5 relative z-10">
            <div class="flex items-baseline gap-1">
              <span class="text-4xl font-black text-white tracking-tight">{{ sensor.latest }}</span>
              <span class="text-lg font-bold text-slate-500">{{ sensor.unit }}</span>
            </div>
            <p class="text-[10px] text-slate-400 mt-1 flex items-center gap-1 font-medium">
              <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
              Atualizado {{ sensor.lastUpdate }}
            </p>
          </div>

          <div class="grid grid-cols-3 gap-2 pt-4 border-t border-slate-800/60 relative z-10">
            <div>
              <span class="block text-[9px] uppercase tracking-wider text-slate-500 font-bold mb-1">Mínima</span>
              <span class="text-sm font-mono text-slate-300">{{ sensor.min }}<span class="text-[10px] text-slate-500 ml-0.5">{{ sensor.unit }}</span></span>
            </div>
            <div class="px-2 border-x border-slate-800/60">
              <span class="block text-[9px] uppercase tracking-wider text-emerald-500 font-bold mb-1">Média</span>
              <span class="text-sm font-mono text-white">{{ sensor.avg }}<span class="text-[10px] text-slate-500 ml-0.5">{{ sensor.unit }}</span></span>
            </div>
            <div class="text-right">
              <span class="block text-[9px] uppercase tracking-wider text-slate-500 font-bold mb-1">Máxima</span>
              <span class="text-sm font-mono text-slate-300">{{ sensor.max }}<span class="text-[10px] text-slate-500 ml-0.5">{{ sensor.unit }}</span></span>
            </div>
          </div>

        </div>
      </div>
    </section>

    <main class="grid grid-cols-1 lg:grid-cols-12 gap-6">
      <section class="lg:col-span-5 bg-slate-900 border border-slate-800 p-6 rounded-3xl flex flex-col justify-between shadow-xl">
        <div>
          <h2 class="text-md font-bold tracking-wide uppercase text-slate-300 mb-4">Simulate Field Telemetry Target</h2>
          
          <div class="mb-4">
            <label class="block text-xs uppercase font-bold text-slate-400 mb-2">Target Telemetry Node</label>
            <select v-model="form.device_id" class="w-full bg-slate-950 border border-slate-800 rounded-xl p-3.5 text-sm font-mono text-slate-200 focus:outline-none focus:border-emerald-500 transition-colors">
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
            <input type="number" step="0.1" v-model.number="form.reading_value" class="w-full bg-slate-950 border border-slate-800 rounded-xl p-3.5 font-mono text-slate-200 text-lg focus:outline-none focus:border-emerald-500 transition-colors" />
          </div>

          <div class="grid grid-cols-3 gap-2 mb-6">
            <button @click="applyTemplate(45.2)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2.5 rounded-xl font-medium transition">Normal Data</button>
            <button @click="applyTemplate(0.0)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2.5 rounded-xl font-medium transition">Sensor Noise</button>
            <button @click="applyTemplate(145.8)" class="bg-slate-950 hover:bg-slate-800 border border-slate-800 text-slate-300 text-xs py-2.5 rounded-xl font-medium transition">Critical Outlier</button>
          </div>
        </div>

        <button @click="dispatchTelemetry" class="w-full bg-slate-100 hover:bg-white text-slate-950 font-bold text-xs uppercase tracking-widest py-4 rounded-xl transition-all shadow-xl active:scale-[0.98]">
          Transmit Payload Matrix
        </button>
      </section>

      <section class="lg:col-span-7 bg-slate-900 border border-slate-800 rounded-3xl flex flex-col h-[420px] shadow-xl">
        <div class="border-b border-slate-800 px-5 py-4 flex justify-between items-center bg-slate-900/50 rounded-t-3xl">
          <span class="text-xs font-bold uppercase tracking-wider text-slate-400 font-mono">Live Infrastructure Logging Terminal</span>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
            <span class="text-[10px] font-mono tracking-widest text-emerald-500 uppercase font-bold">Streaming Active</span>
          </div>
        </div>
        <div ref="terminal" class="p-5 flex-1 overflow-y-auto font-mono text-xs space-y-2 bg-slate-950 rounded-b-3xl scrollbar-thin">
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
      logs: [],
      // Mock tracking dataset simulating distributed IoT mesh components architecture
      sensors: [
        { id: 'ESP32-TEST-001', name: 'Umidade Alpha', type: 'humidity', latest: 45.2, unit: '%', min: 38.5, max: 48.1, avg: 43.0, status: 'VALID', lastUpdate: 'agora mesmo' },
        { id: 'esp32-gate-e50080', name: 'Temperatura Solo', type: 'temperature', latest: 27.4, unit: '°C', min: 22.1, max: 31.0, avg: 26.5, status: 'VALID', lastUpdate: 'há 2 min' },
        { id: 'esp32-gate-7a8d33', name: 'Monitor Irrigação', type: 'water', latest: 12.0, unit: 'L/m', min: 0.0, max: 15.5, avg: 8.2, status: 'VALID', lastUpdate: 'há 5 min' },
        { id: 'esp32-gate-275e00', name: 'Sensor Estufa', type: 'temperature', latest: 85.3, unit: '°C', min: 25.0, max: 88.0, avg: 40.1, status: 'ANOMALY', lastUpdate: 'há 10 seg' }
      ]
    }
  },
  computed: {
    // Maps API raw statistics data onto structured UI display metadata with case/character normalization
    metricsMap() {
      const findCount = (status) => {
        const target = status.toUpperCase().replace('_', '');
        return this.metrics.find(m => {
          const current = m.status.toUpperCase().replace('_', '');
          return current === target || current.includes(target) || target.includes(current);
        })?.count || 0;
      };

      // Inline Heroicons SVG definitions to bypass external resource library overheads
      const iconClock = `<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`;
      const iconCheck = `<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`;
      const iconWifi = `<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636a9 9 0 010 12.728m0 0l-2.829-2.829m2.829 2.829L21 21M15.536 8.464a5 5 0 010 7.072m0 0l-2.829-2.829m-4.243 2.829a4.978 4.978 0 01-1.414-2.83m-1.414 5.658a9 9 0 01-2.167-9.238m7.824 2.167a1 1 0 111.414 1.414m-1.414-1.414L3 3m8.293 8.293l1.414 1.414"></path></svg>`;
      const iconAlert = `<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>`;

      return [
        { label: 'Pending Evaluation', count: findCount('PENDING'), colorClass: 'text-amber-400', iconBgClass: 'bg-amber-500/10 border-amber-500/20', svg: iconClock },
        { label: 'Validated Records', count: findCount('VALID'), colorClass: 'text-emerald-400', iconBgClass: 'bg-emerald-500/10 border-emerald-500/20', svg: iconCheck },
        { label: 'Signal Interference', count: findCount('ANOMALY_NOISE'), colorClass: 'text-sky-400', iconBgClass: 'bg-sky-500/10 border-sky-500/20', svg: iconWifi },
        { label: 'Critical Outliers', count: findCount('ANOMALY_CRITICAL'), colorClass: 'text-rose-500', iconBgClass: 'bg-rose-500/10 border-rose-500/20', svg: iconAlert }
      ];
    },
    // Computes overall operation stability percentage using sanitized payload key strings
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
    // Register execution loop on initial DOM mount lifecycle hook
    this.pollEngine();
    setInterval(this.pollEngine, 2500);
  },
  methods: {
    // Resolves and returns the corresponding raw inline SVG markup corresponding to node properties 
    getSensorIcon(type) {
      const icons = {
        humidity: `<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"></path></svg>`,
        temperature: `<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>`,
        water: `<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path></svg>`
      };
      return icons[type] || icons.temperature;
    },
    // Synchronizes tracking metrics and infrastructure logs directly from backend api
    async pollEngine() {
      try {
        const activeUrl = this.apiUrl;

        const mRes = await fetch(`${activeUrl}/api/v1/dashboard/metrics`);
        if (mRes.ok) {
          const data = await mRes.json();
          this.metrics = data.metrics || [];
        } else {
          console.warn(`Metrics failed with status: ${mRes.status}`);
        }

        const lRes = await fetch(`${activeUrl}/api/v1/dashboard/logs`);
        if (lRes.ok) {
          this.logs = (await lRes.json()).reverse();
          this.$nextTick(this.scrollToBottom);
        } else {
          console.warn(`Logs failed with status: ${lRes.status}`);
        }
      } catch (e) {
        console.error("Erro na comunicação com o Gateway Rust:", e);
      }
    },
    // Submits structured JSON hardware reading simulation down backend streams
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

        this.pollEngine();
      } catch (e) {
        console.error("Erro ao transmitir telemetria:", e);
      }
    },
    // Simulates an Over-The-Air hardware cluster discovery cycle
    triggerOtaScan() {
      this.scanning = true;
      setTimeout(() => {
        this.scanning = false;
        this.pollEngine();
      }, 1200);
    },
    // Preset injector for handling form target evaluation data ranges
    applyTemplate(val) {
      this.form.reading_value = val;
    },
    // Controls bounding frame alignment scrolling inside terminal block
    scrollToBottom() {
      const term = this.$refs.terminal;
      if (term) term.scrollTop = term.scrollHeight;
    },
    // Extracts timestamp values from standard operational strings
    formatTime(str) {
      if (!str) return '';
      return new Date(str).toLocaleTimeString();
    },
    // Dynamically manages CSS styles based on origin server component tags
    getComponentColor(comp) {
      if (!comp) return 'bg-slate-950 text-slate-400';
      if (comp.includes('RUST')) return 'bg-orange-950 text-orange-400 border border-orange-800';
      if (comp.includes('AI')) return 'bg-purple-950 text-purple-400 border border-purple-800';
      return 'bg-blue-950 text-blue-400 border border-blue-800';
    },
    // Assigns distinct theme colors matching log urgency ranks
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
/* Thin scrollbar design styling rules */
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
