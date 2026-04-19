# Guide: Building Your First DDE Dashboard

This guide walks you through the end-to-end process of ingesting raw data and building a self-service analytical dashboard using the Dynamic Data Engine (DDE).

## 1. Prepare the Data
We will use the provided [sensor_telemetry.json](../examples/sensor_telemetry.json) file.

### Example Schema
- `timestamp`: ISO8601 Date
- `sensor_id`: Categorical ID
- `reading`: Numeric Value
- `status`: Categorical Status (ok, warning, error)

---

## 2. Ingest the Data
Use `curl` to upload the sample data into a new dynamic table.

1. **Create Table Schema**:
   ```bash
   curl -X POST http://localhost:8080/v1/tables \
     -H "Content-Type: application/json" \
     -d '{"name": "sensor_data", "description": "High-frequency sensor readings", "config": {}}'
   ```
   *Copy the `id` from the response (e.g. `550e8400-e29b-41d4-a716-446655440000`).*

2. **Upload Data**:
   ```bash
   curl -X POST http://localhost:8081/v1/data/550e8400-e29b-41d4-a716-446655440000 \
     -H "Content-Type: application/json" \
     --data-binary @examples/sensor_telemetry.json
   ```

---

## 3. Administrative Management
To make the data available for analysis, navigate to the **Admin DDE** console.

1. **Toggle Visibility**: Find `sensor_data` in the table and click the **HIDDEN** button to change it to **VISIBLE**.
2. **Optimize Performance**: In the "Add Index" section for the `sensor_data` row:
   - Enter `status` in the input field.
   - Click **Add Index**.
   - This creates a B-TREE index on the JSONB path `payload->>'status'`.

---

## 4. Build the Analysis Dashboard
Navigate to the **Data Explorer** page.

1. **Select Table**: Choose `sensor_data` from the dropdown.
2. **Configure Summary**:
   - **Field**: `reading`
   - **Function**: `AVG`
3. **Add Filters** (The Query Builder):
   - Click **+ Add Filter**.
   - **Field**: `status`
   - **Operator**: `=`
   - **Value**: `ok`
4. **Execute**: Click **Run Analysis**.

### Result
You will see a real-time table showing the average sensor readings for all "ok" status points, bucketed by time interval. This data can be exported as CSV for forensic reporting.

> [!IMPORTANT]
> The **Audit Trail** recorded your administrative actions. To verify the cryptographic integrity of these changes, run `cargo run --bin verify_audit` in the `backend` directory.
