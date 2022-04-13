import requests

USER_ID = "shimo8810"
SUBMISSIONS_URI = "https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions"
PROBLEMS_URI = "https://kenkoooo.com/atcoder/resources/problems.json"
CONTEST_NAMES = ("abc", "arc", "agc")


def generate_readme(progress_info) -> str:
    progress_badges = "\n".join(
        [
            f"![{name.upper()}](https://progress-bar.dev/{nac}/?title={name.upper()}&scale={total}&width=110&suffix=)"  # noqa
            for name, nac, total in progress_info
        ]
    )
    return f"""# AtCoder on Rust
{progress_badges}

AtCoderのRustによる解答
"""


if __name__ == "__main__":
    num_probs = {"abc": 0, "arc": 0, "agc": 0}
    resp = requests.get(PROBLEMS_URI)
    problems = resp.json()
    for prob in problems:
        contest = prob["contest_id"][:3]
        if contest not in CONTEST_NAMES:
            continue
        num_probs[contest] += 1

    resp = requests.get(SUBMISSIONS_URI, params={"user": USER_ID, "from_second": 0})
    submissions = resp.json()

    acs = {"abc": set(), "arc": set(), "agc": set()}
    for submission in submissions:
        if submission["result"] != "AC":
            continue
        contest = submission["problem_id"][:3]
        if contest not in CONTEST_NAMES:
            continue
        acs[contest].add(submission["problem_id"])

    progress_info = [(n, len(acs[n]), num_probs[n]) for n in CONTEST_NAMES]

    readme = generate_readme(progress_info)

    with open("README.md", "w") as f:
        f.write(readme)
