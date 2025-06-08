import { renderRustVisitor } from "@codama/renderers";
import { createFromRoot } from "codama";

import codamaIDL from "../programs/holosim/codamaIDL.json";
// import codamaIDL from "../programs/player_profile/codamaIDL.json";
// import codamaIDL from "../programs/profile_faction/codamaIDL.json";

const codama = createFromRoot(codamaIDL);

codama.accept(renderRustVisitor("./programs/holosim/src/generated", {}));
// codama.accept(renderRustVisitor("./programs/player_profile/src/generated", {}));
// codama.accept(
//   renderRustVisitor("./programs/profile_faction/src/generated", {}),
// );
