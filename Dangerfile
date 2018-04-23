# Display a welcome message for new users
welcome_message.greet

# Sometimes it's a README fix, or something like that - which isn't relevant for
# including in a project's CHANGELOG for example
declared_trivial = github.pr_title.include? "#trivial"

# Mention some potential reviewers, excluding the bot
mention.run(2, [], ["econobot"])

# Make it more obvious that a PR is a work in progress and shouldn't be merged yet
warn("PR is classed as Work in Progress") if github.pr_title.include? "[WIP]"

# Warn when there is a big PR
warn("Big PR") if git.lines_of_code > 500

# Check for/format of Changelog changes.
changelog.check

# Check for conflicts between PRs
conflict_checker.check_conflict_and_comment
