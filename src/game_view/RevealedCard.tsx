// Copyright © spellclash 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { ReactNode } from "react";
import { RevealedCardView } from "../display_types";

export function RevealedCard({
  revealed,
}: {
  revealed: RevealedCardView;
}): ReactNode {
  let borderClass = "border-2 border-black";
  let label = "";
  if (revealed.status?.CanPlay != null) {
    borderClass = "border-4 border-amber-300";
  } else if (revealed.status?.Attacking != null) {
    borderClass = "border-4 border-teal-300";
    label = revealed.status.Attacking;
  } else if (revealed.status?.Blocking != null) {
    borderClass = "border-4 border-purple-300";
    label = revealed.status.Blocking;
  }

  return (
    <div className={borderClass}>
      <img
        src={revealed.face.image}
        style={{
          width: "100%",
          height: "100%",
        }}
      />
    </div>
  );
}
