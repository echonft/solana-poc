const { createFromIdls, renderJavaScriptVisitor, updateProgramsVisitor, setAnchorDiscriminatorsVisitor } = require(
  "@metaplex-foundation/kinobi",
);
const { join } = require("path");

// Instantiate Kinobi.
const kinobi = createFromIdls([
  join(__dirname, "target", "idl", "echo.json"),
]);

// Update the Kinobi tree using visitors...
kinobi.update(
  updateProgramsVisitor({
    "echo": {
      publicKey: "FNwASR1r8FD9HA4beTDJnf5yAJkNYkFB9Wa1QwbV8v1P",
      origin: "anchor",
    },
  }),
);
kinobi.update(
  setAnchorDiscriminatorsVisitor(),
);

// Render JavaScript.
const jsDir = join(__dirname, "target", "sdk");
kinobi.accept(renderJavaScriptVisitor(jsDir));
