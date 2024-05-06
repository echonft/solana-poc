import {
  createFromJson,
  type Node,
  renderJavaScriptVisitor,
  setAnchorDiscriminatorsVisitor,
  updateProgramsVisitor,
  type Visitor
} from '@metaplex-foundation/kinobi'
import { join } from 'path'
import EchoIdl from 'target/idl/echo.json'

const __dirname = import.meta.dirname
// Instantiate Kinobi.
const kinobi = createFromJson(JSON.stringify(EchoIdl))

// Update the Kinobi tree using visitors...
kinobi.update(
  updateProgramsVisitor({
    'echo': {
      publicKey: 'Gzq13nAmkDZMgFjKs8Zd6jJbXE2X6iJJ4FEe2BqWcVWM',
      origin: 'anchor'
    }
  })
)
kinobi.update(setAnchorDiscriminatorsVisitor() as Visitor<Node>)

// Render JavaScript.
const jsDir = join(__dirname, 'target', 'sdk')
kinobi.accept(renderJavaScriptVisitor(jsDir) as Visitor<void>)
