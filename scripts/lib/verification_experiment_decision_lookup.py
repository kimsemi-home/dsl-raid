def experiment_records(experiments):
    return {row.get("id", ""): row for row in experiments.get("experiments", [])}


def decision_records(data):
    return {row.get("experiment", ""): row for row in data.get("decisions", [])}


def check_promoted_experiments(records, decisions, errors):
    for exp in records.values():
        if exp.get("promoted") and exp.get("id", "") not in decisions:
            errors.append(f"{exp.get('id', '')} missing decision")
