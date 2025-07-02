# Eddi-pad: High-Performance Simulation Environment

## Vision Statement

**Eddi-pad** is a high-performance, real-time simulation environment that bridges virtual design and physical deployment for interactive installations. It provides an intuitive interface for designing responsive environments while serving as a performant I/O control hub for reaching real sound, light, Bluetooth, and other output targets.

## Core Philosophy

- **Performance First**: Sub-millisecond latency for device communication
- **Device Agnostic**: Unified interface for virtual and physical targets  
- **Spatial Intelligence**: Leverages Eddi's proven 7-zone spatial model
- **Movement Responsive**: Deep integration with StreamPoseML pose analysis
- **Real-time Visualization**: Live preview of all environmental effects

## Why Rust?

### Performance Requirements
- **Device Protocol Handling**: Simultaneous management of 100+ devices across multiple protocols
- **Real-time Constraints**: <1ms latency for critical lighting/sound cues
- **Memory Safety**: Zero-cost abstractions without garbage collection pauses
- **Concurrency**: Efficient handling of multiple device streams simultaneously

### Ecosystem Advantages
- **Cross-platform**: Native performance on macOS, Linux, Windows
- **WASM Support**: Browser-based UI components with native performance
- **C Interop**: Easy integration with existing DMX/OSC libraries
- **Growing Ecosystem**: Strong support for multimedia and device protocols

## Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Eddi-pad UI   │◄──►│  Eddi-pad Core   │◄──►│ Physical        │
│   (2D Spatial)  │    │  (Rust Engine)   │    │ Devices         │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                         
         │              ┌────────▼────────┐                
         │              │ Virtual Device  │                
         │              │ Simulation      │                
         │              └─────────────────┘                
         │                       ▲                         
         │                       │                         
         ▼               ┌───────┴───────┐                 
┌─────────────────┐     │               │     ┌─────────────────┐
│ Simulated Poses │     │               │     │      Eddi       │
│ & Test Data     │     │               │     │ (AI Director +  │
└─────────────────┘     │               │     │ Spatial Logic)  │
         │               │               │     └─────────────────┘
         └──────────────►│               │◄──────────▲            
                         │               │           │            
┌─────────────────┐      │               │     ┌─────▼───────────┐
│  StreamPoseML   │◄────►│  Data Flow    │     │ Gesture Events  │
│ (Pose Analysis) │      │  & Control    │     │ & Spatial       │
│                 │      │  Hub          │     │ Responses       │
└─────────────────┘      │               │     └─────────────────┘
         ▲               │               │                        
         │               │               │                        
┌────────▼────────┐      │               │     ┌─────────────────┐
│ Input Devices   │      │               │     │  Skeleton-MHI   │
│ (Cameras,       │      │               │◄────│ (Novel Gesture  │
│  Kinect, etc.)  │      │               │     │  Segmentation)  │
└─────────────────┘      └───────────────┘     └─────────────────┘
```

### Data Flow Pipeline
```
Input → StreamPoseML → Skeleton-MHI → Eddi → Eddi-pad → Devices
Devices  (pose data)   (gesture      (spatial (device   (output)
                       segments)     response) control)
```

### Core Components

#### 1. Device Abstraction Layer (Rust)
```rust
trait Device {
    fn set_parameter(&mut self, param: &str, value: f32) -> Result<(), DeviceError>;
    fn get_status(&self) -> DeviceStatus;
    fn get_latency(&self) -> Duration;
}

// Virtual devices for simulation
struct VirtualLight { ... }
struct VirtualSpeaker { ... }

// Physical device adapters
struct DMXDevice { ... }
struct OSCDevice { ... }
struct BluetoothDevice { ... }
```

#### 2. Spatial Engine
- Extends Eddi's 7-zone model to arbitrary 2D spaces
- Real-time spatial mapping and device assignment
- Physics-based light/sound propagation simulation

#### 3. Protocol Handlers
- **DMX512**: Professional lighting control
- **OSC**: Audio/visual software integration  
- **Bluetooth**: StreamPoseML device connectivity
- **WebSocket**: Real-time browser communication
- **UDP/TCP**: Custom protocol support

#### 4. Simulation Engine
- Real-time 2D visualization using `egui`
- Virtual device physics (light falloff, sound propagation)
- Performance profiling and optimization tools

## Key Features

### 1. Unified Device Management
```rust
let mut space = EddiSpace::new();

// Add virtual devices for design
space.add_device(VirtualLight::new("front_left", Position::new(2.0, 3.0)));

// Add physical devices for deployment  
space.add_device(DMXLight::new("channel_1", DMXConfig::default()));

// Same interface for both
space.set_zone_color("front", Color::rgb(255, 0, 0));
```

### 2. Real-time Spatial Mapping
- Drag-and-drop device placement in 2D space
- Automatic zone assignment based on position
- Live preview of lighting/sound effects
- Export configurations for physical deployment

### 3. Movement Integration
```rust
// StreamPoseML pose data integration
let pose_data = streampose_client.get_latest_pose()?;
let gesture = gesture_analyzer.analyze(&pose_data)?;

// Drive environmental response
if gesture.is_expansive() {
    space.increase_arousal(0.2);
    space.expand_lighting_to_periphery();
}
```

### 4. Performance Monitoring
- Real-time latency measurement per device
- Throughput monitoring (commands/second)
- Memory usage tracking
- Frame rate optimization

## Integration Strategy

### With Existing Eddi System
- **Direct Integration**: Rust-based replacement for `spatial_light_controller.py`
- **Protocol Bridge**: WebSocket/OSC bridge for gradual migration
- **Config Compatibility**: Read existing `spatial_device_configuration.json`

### With StreamPoseML & Skeleton-MHI
- **WebSocket Client**: Subscribe to pose data streams from StreamPoseML
- **Gesture Event Streaming**: Receive segmented gestures from Skeleton-MHI
- **Bidirectional Simulation**: Generate synthetic pose data for testing the full pipeline
- **Feature Visualization**: Display MHI energy patterns and gesture boundaries in 2D interface

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Device Command Latency | <1ms | Imperceptible delay for lighting |
| Pose Processing | <10ms | Real-time movement response |
| Concurrent Devices | 1000+ | Large installation support |
| Memory Usage | <100MB | Efficient resource utilization |
| CPU Usage | <25% | Leave headroom for other processes |

## Development Roadmap

### Phase 1: Core Engine (4-6 weeks)
- Rust device abstraction layer
- Basic DMX/OSC protocol support
- Simple 2D visualization engine
- WebSocket API for browser UI

### Phase 2: Spatial Intelligence (3-4 weeks)  
- 2D space modeling and zone mapping
- Device placement and assignment algorithms
- Physics-based simulation (light falloff, etc.)
- Performance profiling tools

### Phase 3: Integration (3-4 weeks)
- StreamPoseML WebSocket client
- Eddi Director system integration
- Bluetooth device management
- Configuration import/export

### Phase 4: Advanced Features (4-6 weeks)
- Advanced gesture recognition
- Machine learning model integration
- Scenario designer UI
- Real-time collaboration features

## Technical Specifications

### Protocol Support Matrix
| Protocol | Read | Write | Bidirectional | Latency Target |
|----------|------|-------|---------------|----------------|
| DMX512   | ✓    | ✓     | ✗             | <500μs         |
| OSC      | ✓    | ✓     | ✓             | <1ms           |
| Bluetooth| ✓    | ✓     | ✓             | <10ms          |
| WebSocket| ✓    | ✓     | ✓             | <5ms           |
| Serial   | ✓    | ✓     | ✓             | <2ms           |

## Development Status

🚧 **Early Development** - Following TDD methodology as defined in [CLAUDE.md](../CLAUDE.md)

## Related Projects

This project is designed to integrate with:
- [StreamPoseML](../StreamPoseML/) - Real-time pose extraction and feature engineering
- [Skeleton-MHI](../skeleton-mhi/) - Novel gesture segmentation using keypoint-based Motion History Images
- [Eddi](../eddi/) - Movement-responsive environment system with AI Director

## Conclusion

Eddi-pad represents a significant evolution in interactive environment control, combining the spatial intelligence of Eddi with the movement analysis of StreamPoseML in a high-performance, unified platform. By leveraging Rust's performance and safety guarantees, we can create a tool that satisfies both the creative needs of designers and the technical requirements of professional installations.

The modular architecture ensures that Eddi-pad can grow with the ecosystem while maintaining backwards compatibility with existing workflows. This positions it as both a powerful standalone tool and a central hub for the broader interactive environment ecosystem.