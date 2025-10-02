import * as textRunner from "text-runner"
import { execFile } from 'node:child_process';

export function call(action: textRunner.actions.Args, done: textRunner.exports.DoneFunction) {
  action.name("verify subcommand")
  const args = action.region.text().split(" ")
  switch (args.length) {
    case 0:
      throw new Error("empty block")
    case 1:
      throw new Error("no args given")
    case 2:
      action.name(`verify subcommand "${args[1]}"`)
      validate_subcommand(args[0], args[1], done)
      break
    case 3:
      validate_subcommand_flag(args)
      break
    default:
      throw new Error("too many args: "+args.length)
   }
}

async function validate_subcommand(executable: string, subcommand: string, done: textRunner.exports.DoneFunction) {
   execFile("../../target/debug/cucumber-sort", [subcommand, "--help"], (err: Error | null, stdout: string, stderr: string) => {
    if (err == null) {
      done()
    } else {
      console.log(stdout)
      console.log(stderr)
      done(new Error(`${subcommand} seems not a valid subcommand for ${executable}`))
    }
  })
}

async function validate_subcommand_flag(args: string[]) {}
