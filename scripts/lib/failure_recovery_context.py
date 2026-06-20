def build_context(failure, incident, learning, evidence):
    return {
        "responses": failure_responses(failure),
        "incidents": known_incidents(incident, learning),
        "updates": known_updates(incident, learning),
        "revalidations": known_revalidations(incident, learning),
        "outputs": generated_outputs(evidence),
    }


def failure_responses(failure):
    return {row["id"]: row["response"] for row in failure["conditions"]}


def known_incidents(incident, learning):
    values = {row["id"] for row in incident["cycles"]}
    values |= {row["incident"] for row in learning["cycles"]}
    return values


def known_updates(incident, learning):
    values = {row["knowledge_update"] for row in incident["cycles"]}
    values |= {row["knowledge_update"] for row in learning["cycles"]}
    return values


def known_revalidations(incident, learning):
    values = {row["revalidation"] for row in incident["cycles"]}
    values |= {row["revalidation"] for row in learning["cycles"]}
    return values


def generated_outputs(evidence):
    return {row["output"] for row in evidence["generated_backends"]}
