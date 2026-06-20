def generated_outputs(evidence):
    return {row.get("output", "") for row in evidence.get("generated_backends", [])}


def check_output_refs(row, outputs, errors):
    for field in ("inputs", "required_verification", "evidence"):
        for item in row.get(field, []):
            if item not in outputs:
                errors.append(f"{row.get('id', '')} unknown {field} {item}")
