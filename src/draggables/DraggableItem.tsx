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

import React, { useEffect } from 'react';
import classNames from 'classnames';
import { type DraggableSyntheticListeners, type UniqueIdentifier } from '@dnd-kit/core';
import type { Transform } from '@dnd-kit/utilities';

import styles from './DraggableItem.module.css';
import { useSortable } from '@dnd-kit/sortable';
import { useState } from 'react';
import { ReactNode } from 'react';

interface Props {
  readonly id: UniqueIdentifier;
  readonly children: ReactNode;
  readonly index?: number;
  readonly disabled?: boolean;
  readonly dragOverlay?: boolean;
}

export function DraggableItem({ id, children, index, disabled, dragOverlay }: Props) {
  const { setNodeRef, listeners, isDragging, isSorting, transform, transition } = useSortable({
    id,
  });
  const mounted = useMountStatus();
  const mountedWhileDragging = isDragging && !mounted;

  return (
    <InnerItem
      ref={disabled || dragOverlay ? undefined : setNodeRef}
      dragging={isDragging}
      dragOverlay={dragOverlay}
      sorting={isSorting}
      index={index}
      transition={transition}
      transform={transform}
      fadeIn={mountedWhileDragging}
      listeners={listeners}
    >
      {children}
    </InnerItem>
  );
}

function useMountStatus() {
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    const timeout = setTimeout(() => setIsMounted(true), 500);

    return () => clearTimeout(timeout);
  }, []);

  return isMounted;
}

interface InnerProps {
  dragOverlay?: boolean;
  disabled?: boolean;
  dragging?: boolean;
  index?: number;
  fadeIn?: boolean;
  transform?: Transform | null;
  listeners?: DraggableSyntheticListeners;
  sorting?: boolean;
  transition?: string | null;
  children: React.ReactNode;
}

/* eslint-disable react/prop-types */
const InnerItem = React.memo(
  React.forwardRef<HTMLLIElement, InnerProps>(
    (
      {
        dragOverlay,
        dragging,
        disabled,
        fadeIn,
        index,
        listeners,
        sorting,
        transition,
        transform,
        children,
        ...props
      },
      ref,
    ) => {
      useEffect(() => {
        if (!dragOverlay) {
          return;
        }

        document.body.style.cursor = 'grabbing';

        return () => {
          document.body.style.cursor = '';
        };
      }, [dragOverlay]);

      return (
        <li
          className={classNames(
            styles.Wrapper,
            fadeIn && styles.fadeIn,
            sorting && styles.sorting,
            dragOverlay && styles.dragOverlay,
          )}
          style={
            {
              transition: [transition].filter(Boolean).join(', '),
              '--translate-x': transform ? `${Math.round(transform.x)}px` : undefined,
              '--translate-y': transform ? `${Math.round(transform.y)}px` : undefined,
              '--scale-x': transform?.scaleX ? `${transform.scaleX}` : undefined,
              '--scale-y': transform?.scaleY ? `${transform.scaleY}` : undefined,
              '--index': index,
            } as React.CSSProperties
          }
          ref={ref}
        >
          <div
            className={classNames(
              styles.Item,
              dragging && styles.dragging,
              dragOverlay && styles.dragOverlay,
              disabled && styles.disabled,
            )}
            data-cypress="draggable-item"
            {...listeners}
            {...props}
            tabIndex={0}
          >
            {children}
          </div>
        </li>
      );
    },
  ),
);
