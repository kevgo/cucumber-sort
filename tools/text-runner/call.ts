import * as textRunner from "text-runner"
import { execFile } from 'node:child_process';

export async function call (action: textRunner.actions.Args) {
  action.name("verify subcommand")
  const args = action.region.text().split(" ")
  switch (args.length) {
    case 0:
      throw new Error("empty block")
    case 1:
      throw new Error("no args given")
    case 2:
      validate_binary(args[0])
      if (args[1].startsWith("-")) {
        throw new Error("top-level flag")
      }
      action.name(`verify subcommand "${args[1]}"`)
      validate_subcommand(args[0], args[1])
      break
    case 3:
      validate_subcommand_flag(args)
      break
    default:
      throw new Error("too many args: "+args.length)
   }
}

async function validate_subcommand(executable: string, subcommand: string) {
  const result = await execFile("../target/debug/cucumber-sort", [subcommand, "-h"])
}

async function validate_subcommand_flag(args: string[]) {}

function validate_binary(binary: string) {
  if (binary !== "cucumber-sort") {
    throw new Error("can only test cucumber-sort");
  }
}
