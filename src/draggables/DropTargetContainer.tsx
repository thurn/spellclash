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

import React, { forwardRef } from 'react';
import classNames from 'classnames';

import styles from './DropTargetContainer.module.css';
import {
  AnimateLayoutChanges,
  SortableContext,
  defaultAnimateLayoutChanges,
  horizontalListSortingStrategy,
  useSortable,
} from '@dnd-kit/sortable';
import { UniqueIdentifier } from '@dnd-kit/core';

interface Props {
  readonly id: UniqueIdentifier;
  readonly children: React.ReactNode;
  readonly items: UniqueIdentifier[];
  readonly height: string;
}

export function DropTargetContainer({ children, id, items, ...props }: Props) {
  const { setNodeRef } = useSortable({
    id,
    data: {
      type: 'container',
      children: items,
    },
    animateLayoutChanges,
  });
  return (
    <InnerContainer ref={setNodeRef} {...props}>
      <SortableContext items={items} strategy={horizontalListSortingStrategy}>
        {children}
      </SortableContext>
    </InnerContainer>
  );
}

const animateLayoutChanges: AnimateLayoutChanges = (args) =>
  defaultAnimateLayoutChanges({ ...args, wasDragging: true });

interface InnerContainerProps {
  readonly children: React.ReactNode;
  readonly height: string;
}

/* eslint-disable react/display-name */
const InnerContainer = forwardRef<HTMLDivElement, InnerContainerProps>(
  ({ children, height, ...props }: InnerContainerProps, ref) => {
    return (
      <div
        {...props}
        ref={ref}
        style={{
          height,
        }}
        className={classNames(styles.Container)}
      >
        {<ul style={{ flex: 1 }}>{children}</ul>}
      </div>
    );
  },
);
