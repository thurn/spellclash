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

import { ReactNode } from 'react';
import { MainMenuView } from './generated_types';
import { GameButton } from './core/GameButton';

function MainMenu({ view }: { view: MainMenuView }): ReactNode {
  const imageHeight = 125;
  const imageAspectRatio = 1.4;
  return (
    <div className="w-screen h-screen">
      <img
        style={{
          transformOrigin: 'center',
          transform: 'translateY(-15%) rotate(90deg)',
          position: 'absolute',
          left: 0,
          right: 0,
          marginLeft: 'auto',
          marginRight: 'auto',
          width: `${imageHeight}vh`,
          height: `${imageHeight * imageAspectRatio}vh`,
          zIndex: -10,
        }}
        src="https://cards.scryfall.io/png/front/2/3/23c4e8fb-0bc2-4449-a8df-a455b1ea9be4.png"
      />
      <MenuItems view={view} />
      <Attribution />
    </div>
  );
}

function MenuItems({ view }: { view: MainMenuView }): ReactNode {
  const buttons = view.buttons.map((button, i) => <GameButton button={button} key={i} className="m-2" />);
  return (
    <div className="flex flex-col w-1/5 items-stretch text-center absolute left-2 bottom-2">
      <h1 className="text-3xl font-bold text-white font-title">Spellclash</h1>
      {buttons}
    </div>
  );
}

function Attribution(): ReactNode {
  return (
    <div className="flex flex-col items-stretch text-center absolute right-2 bottom-2">
      <h1 className="text-s text-slate-300">&ldquo;Brotherhood&apos;s End&rdquo; by Bryan Sola</h1>
    </div>
  );
}

export default MainMenu;
