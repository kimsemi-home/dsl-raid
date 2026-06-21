def generated_outputs(evidence):
    return {row.get("output", "") for row in evidence.get("generated_backends", [])}


def check_generated_reference(rid, item, outputs, errors):
    if item not in outputs:
        errors.append(f"unknown evidence {rid} {item}")


def check_output_reference(row, outputs, errors):
    rid = row.get("id", "")
    output = row.get("output", "")
    if output not in outputs:
        errors.append(f"unknown output {rid} {output}")
