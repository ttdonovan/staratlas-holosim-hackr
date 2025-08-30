import fs from "node:fs";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import {
  createFromRoot,
  updateInstructionsVisitor,
  type ProgramUpdates,
} from "codama";

import anchorIdl from "../programs/holosim/SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9-idl.json";
// import anchorIdl from "../programs/player_profile/player_profile.json";
// import anchorIdl from "../programs/profile_faction/profile_faction.json";

const codama = createFromRoot(rootNodeFromAnchor(anchorIdl));

// TODO: will need to revisit this later...arg errors with codama renderer
const map: Record<string, ProgramUpdates> = {
  closeCraftingProcess: { delete: true },
  scanForSurveyDataUnits: { delete: true },
  stopMiningAsteroid: { delete: true },
};

// // TODO: will need to revisit this later...arg errors with codama renderer
// const map: Record<string, ProgramUpdates> = {
//   setName: { delete: true },
//   setRoleName: { delete: true },
// };

codama.update(updateInstructionsVisitor(map));

fs.writeFileSync("./programs/holosim/codamaIDL.json", codama.getJson());
// fs.writeFileSync("./programs/player_profile/codamaIDL.json", codama.getJson());
// fs.writeFileSync("./programs/profile_faction/codamaIDL.json", codama.getJson());
