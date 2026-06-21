def generated_outputs(evidence):
    return {row.get("output", "") for row in evidence.get("generated_backends", [])}


def check_evidence_refs(row, outputs, errors):
    rid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{rid} must cite evidence")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"unknown evidence {rid} {item}")
