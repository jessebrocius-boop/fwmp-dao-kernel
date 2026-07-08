import json
import hashlib
import time
from typing import Dict, Tuple

class InterfaceGenesisEngine:
    def __init__(self, sales_volume_trigger: float = 0.90):
        """
        Initializes the Dynamic Interface Genesis Pipeline.
        
        :param sales_volume_trigger: The threshold ratio required to spin off a sovereign UI.
        """
        self.trigger_ratio = sales_volume_trigger

    def compile_layout_spec(self, node_id: int, state_metrics: Dict) -> Tuple[bool, str]:
        """
        Evaluates node telemetry. If the structural triggers are achieved,
        it compiles an immutable, state-driven UI specification.
        """
        sales_ratio = state_metrics.get("sales_volume_ratio", 0.0)
        invariant_secure = state_metrics.get("scqos_invariant_secure", False)

        # MCE Check: Do not generate an interface if the sovereign threshold is unmet
        if sales_ratio < self.trigger_ratio:
            return False, json.dumps({
                "status": "HALT",
                "reason": f"Node {node_id} sales ratio ({sales_ratio:.2f}) below sovereign trigger ({self.trigger_ratio:.2f})."
            })

        if not invariant_secure:
            return False, json.dumps({
                "status": "HALT",
                "reason": f"Node {node_id} has a shattered or unstable SCQOS invariant state. Genesis denied."
            })

        # Construct the minimal, component-isolated UI layout matrix
        ui_blueprint = {
            "genesis_timestamp": time.time(),
            "target_node": node_id,
            "canvas_configuration": {
                "dimensions": "8x8_glyph_grid",
                "refresh_strategy": "passive_interrupt_only"
            },
            "components": [
                {
                    "id": "matrix_proof_display",
                    "type": "SymmetryGrid",
                    "source_vector": f"Node_{node_id}_state_hash"
                },
                {
                    "id": "triage_action_button",
                    "type": "Trigger",
                    "action_vector": f"arbitrator.execute_triage({node_id})"
                }
            ]
        }

        # Cryptographically bind the UI layout to the generating state metrics
        payload_bytes = json.dumps(ui_blueprint, sort_keys=True).encode('utf-8')
        ui_blueprint["layout_fingerprint"] = hashlib.sha256(payload_bytes).hexdigest()

        return True, json.dumps(ui_blueprint, indent=2)

# --- Verification Flight ---
if __name__ == "__main__":
    engine = InterfaceGenesisEngine(sales_volume_trigger=0.90)
    
    # Context A: Node has volume but is structurally unstable
    unstable_metrics = {"sales_volume_ratio": 0.95, "scqos_invariant_secure": False}
    # Context B: Node clears all conditions for sovereign spin-off
    nominal_sovereign_metrics = {"sales_volume_ratio": 0.92, "scqos_invariant_secure": True}
    
    print("Testing Dynamic Interface Genesis Pipeline...")
    print("=" * 80)
    
    success, output = engine.compile_layout_spec(node_id=101, state_metrics=nominal_sovereign_metrics)
    print(f"Genesis Status: {success}\nCompiled Blueprint:\n{output}")
    print("=" * 80)
