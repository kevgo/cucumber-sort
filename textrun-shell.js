import * as path from "path"
import * as url from "url"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url))
const cucumber_sort_path = path.join(__dirname, "target", "debug", "cucumber-sort")

export default {
  binaries: {
    "cucumber-sort": cucumber_sort_path
  }
}
