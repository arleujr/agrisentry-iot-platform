## AgriSentry IoT Platform - Frontend Dashboard
This repository contains the frontend web application for the AgriSentry Ecosystem, a platform designed for agricultural telemetry monitoring. Built with Vue.js, this dashboard visualizes sensor metrics, tracks device communication, and provides interface components to display hardware status logs.
The client application interacts with the specialized infrastructure nodes of the architecture, which include the Rust high-performance telemetry ingestion gateway and the Python background workers handling data validation.
The live application is deployed and accessible at: https://agrisentry-dashboard.onrender.com/
## Key Features

* Real-time telemetry visualization for agricultural indicators including soil moisture, ambient temperature, and relative humidity.
* Interactive tables detailing data ingestion history and device communication status.
* Responsive layouts adapted for field operations via mobile devices and tablets.

## Tech Stack

* Framework: Vue.js
* State Management: Vuex
* Router: Vue Router
* Styles: CSS3 and Tailwind CSS
* Deployment Environment: Render

## Local Setup
Ensure Node.js is configured on your local machine before starting the installation.

npm install
npm run serve
npm run build

## Platform Architecture Connections
To review the underlying engine and ingestion layers of this ecosystem, refer to the related system repositories:

* Ingestion Gateway: [agrisentry-iot-gateway](https://github.com/arleujr/agrisentry-iot-gateway)
* Processing Engine: [agrisentry-core](https://github.com/arleujr/agrisentry-core)
