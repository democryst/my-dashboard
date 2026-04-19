# Examples & Validation Suite

The `examples/` directory contains sample datasets and standalone validation utilities designed to demonstrate the platform's security, accuracy, and self-service analytical capabilities.

---

## 📊 Sample Datasets

### [sensor_telemetry.json](../examples/sensor_telemetry.json)
A simulated dataset for IoT sensor readings.
- **Fields**: `timestamp`, `sensor_id`, `reading`, `status` (`ok`, `warning`, `error`).
- **Use Case**: Testing the Dynamic Data Engine (DDE) ingestion and visual query builder.
- **How to use**: Follow the [DDE Walkthrough Guide](./DDE_GUIDE.md) to upload this data and build a dashboard.

---

## 🛠 Validation Utilities

These utilities are standalone programs that verify the core technical principles of the system without requiring a full environment setup.

### [validate_dde.rs](../examples/validate_dde.rs)
**Purpose**: Proves the mathematical accuracy of the aggregation engine.
- **Action**: Generates 10,000 synthetic records and calculates a 5-minute average.
- **Proof**: Mathematically verifies that the result matches theoretical expectations to within +/- 0.0001%.
- **How to run**:
  ```bash
  cargo run --example validate_dde
  ```

### [validate_integrity.rs](../examples/validate_integrity.rs)
**Purpose**: Demonstrates the cryptographic audit log and tamper-detection.
- **Action**: 
  1. Generates a valid SHA-256 hash chain of audit logs.
  2. Verifies the chain's integrity.
  3. **Simulates a breach** by modifying a record (e.g., changing an actor's name).
  4. Attempts to re-verify the chain and detects the violation.
- **Proof**: Demonstrates SOC2 compliance for non-repudiable auditing.
- **How to run**:
  ```bash
  cargo run --example validate_integrity
  ```

---

## 🧪 Quick Demo Script

To quickly demonstrate the platform to a technical audience:

1. **Show Security**: Run `cargo run --example validate_integrity` to show how unauthorized data alteration is instantly detected.
2. **Show Accuracy**: Run `cargo run --example validate_dde` to show the precision of the time-bucketed aggregation engine.
3. **Show Discovery**: Upload `sensor_telemetry.json` via the UI and build a live dashboard as described in the [DDE Guide](./DDE_GUIDE.md).
