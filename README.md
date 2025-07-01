# Eddi-pad

A high-performance, real-time simulation environment for interactive installations, built in Rust.

## Overview

Eddi-pad bridges virtual design and physical deployment for responsive environments. It provides an intuitive 2D spatial interface for designing interactive installations while serving as a performant I/O control hub for real sound, light, Bluetooth, and other output targets.

## Key Features

- **High Performance**: Sub-millisecond latency device communication
- **2D Spatial Design**: Intuitive diagram-based space configuration  
- **Device Agnostic**: Unified interface for virtual and physical targets
- **Protocol Support**: DMX512, OSC, Bluetooth, WebSocket
- **Real-time Integration**: StreamPoseML pose data and Eddi spatial intelligence

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Web UI        │◄──►│  Rust Core       │◄──►│ Physical        │
│   (2D Spatial)  │    │  Engine          │    │ Devices         │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │ Virtual Device   │
                       │ Simulation       │
                       └──────────────────┘
```

## Performance Targets

- Device Command Latency: <1ms
- Concurrent Devices: 1000+
- Memory Usage: <100MB
- 2D Rendering: 60+ FPS

## Development Status

🚧 **Early Development** - Core architecture and protocol research phase

## Related Projects

This project is designed to integrate with:
- [Eddi](../eddi/) - Movement-responsive environment system
- [StreamPoseML](../StreamPoseML/) - Real-time pose classification toolkit

## License

[License details to be determined]