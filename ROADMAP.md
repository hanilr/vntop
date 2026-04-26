# Roadmap of Variation Table of Processes
This project is in early development. This roadmap contains features, bugs (if found) and algorithm changes.

---

### User Interactions and Core Features

- [X] Live loop: *Refresh event.*
- [X] Used and Available Sizes as KB: *For now this values used as bytes. Should be in kb.*
- [X] Keybinds: *User actions.*
- [X] Terminal Resize: *When terminal resized update visuals.*
- [ ] Visual Indicators: *Usage percentage bars etc. Probably will add with ratatui*
- [X] Processes visual update: *Should be processes widget more detailed and sorted.*
- [ ] Use `ratatui` for UI: In long term ui should be change to ratatui.
- [X] Removable disks should shows in disks: *Live loop should display this information.*

---

### Data Sorting and Searching

- [X] Process Sorting: *Cpu, memory etc. usage.*
- [ ] Process Searching: *User activity*

---

### Optimization

- [X] Multitasking support: *Refresh event should be doing as multitasking.*
- [ ] Configuration JSON files support: *Change `.txt` to `.json`.*
- [ ] Memory allocation optimization: *Some of string types should be optimized.*

---

### Tests and Benchs

- [X] Make test: *Tests for making optimization easy.*
- [ ] Make bench: *For now bench only for nightly version of rust.*

---

### Future Ideas

- [ ] Power Draw
- [ ] Voltage Sensors
- [ ] Virtual Interface Tracking
- [ ] Latency and Packet Loss Monitor
- [ ] Custom Shell Scripts Integration

---
