import * as textRunner from "text-runner"

export function call (action: textRunner.actions.Args) {
  console.log("This is the implementation of the call action.")
  console.log('Text inside the semantic document region:', action.region.text())
  console.log("For more information see")
  console.log("https://github.com/kevgo/text-runner/blob/main/documentation/user-defined-actions.md")
}
