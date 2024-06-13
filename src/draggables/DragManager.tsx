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

import { useCallback, useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import {
  closestCenter,
  pointerWithin,
  rectIntersection,
  CollisionDetection,
  DndContext,
  DragOverlay,
  DropAnimation,
  getFirstCollision,
  KeyboardSensor,
  MouseSensor,
  TouchSensor,
  useSensors,
  useSensor,
  MeasuringStrategy,
  defaultDropAnimationSideEffects,
  UniqueIdentifier,
} from '@dnd-kit/core';
import { arrayMove } from '@dnd-kit/sortable';
import { coordinateGetter as multipleContainersCoordinateGetter } from './multipleContainersKeyboardCoordinates';

import { DraggableItem } from './DraggableItem';
import { createContext } from 'react';
import { ReactNode } from 'react';

export default {
  title: 'Presets/Sortable/Multiple Containers',
};

export type ContainerId = string;
export type ItemId = string;

// Map of ContainerId -> Array of Item ID
export type Items = Record<ContainerId, ItemId[]>;

export const ItemsContext = createContext<Items>({});

interface Props {
  readonly items: Items;
  readonly children: ReactNode;

  /**
   * Given an item ID, render its contents.
   *
   * Used to implement the drag overlay.
   */
  renderItem(item: ItemId): ReactNode;
}

export function DragManager({ items, children, renderItem }: Props) {
  const coordinateGetter = multipleContainersCoordinateGetter;
  const [itemList, setItemList] = useState<Items>(() => items);
  const [activeId, setActiveId] = useState<ItemId | null>(null);
  const lastOverId = useRef<ContainerId | null>(null);
  const recentlyMovedToNewContainer = useRef(false);

  /**
   * Custom collision detection strategy optimized for multiple containers
   *
   * - First, find any droppable containers intersecting with the pointer.
   * - If there are none, find intersecting containers with the active draggable.
   * - If there are no intersecting containers, return the last matched intersection
   *
   */
  const collisionDetectionStrategy: CollisionDetection = useCallback(
    (args) => {
      if (activeId && activeId in itemList) {
        return closestCenter({
          ...args,
          droppableContainers: args.droppableContainers.filter(
            (container) => container.id in itemList,
          ),
        });
      }

      // Start by finding any intersecting droppable
      const pointerIntersections = pointerWithin(args);
      const intersections =
        pointerIntersections.length > 0
          ? // If there are droppables intersecting with the pointer, return those
            pointerIntersections
          : rectIntersection(args);
      let overId = getFirstCollision(intersections, 'id');

      if (overId != null) {
        if (overId in itemList) {
          const containerItems = itemList[overId];

          // If a container is matched and it contains items (columns 'A', 'B', 'C')
          if (containerItems.length > 0) {
            // Return the closest droppable within that container
            overId = closestCenter({
              ...args,
              droppableContainers: args.droppableContainers.filter(
                (container) =>
                  container.id !== overId && containerItems.includes(toContainerId(container.id)),
              ),
            })[0]?.id;
          }
        }

        lastOverId.current = toContainerId(overId);

        return [{ id: overId }];
      }

      // When a draggable item moves to a new container, the layout may shift
      // and the `overId` may become `null`. We manually set the cached `lastOverId`
      // to the id of the draggable item that was moved to the new container, otherwise
      // the previous `overId` will be returned which can cause items to incorrectly shift positions
      if (recentlyMovedToNewContainer.current) {
        lastOverId.current = activeId;
      }

      // If no droppable is matched, return the last match
      return lastOverId.current ? [{ id: lastOverId.current }] : [];
    },
    [activeId, itemList],
  );

  const [clonedItems, setClonedItems] = useState<Items | null>(null);

  const sensors = useSensors(
    useSensor(MouseSensor),
    useSensor(TouchSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter,
    }),
  );

  const findContainerId = (id: ContainerId | ItemId) => {
    if (id in itemList) {
      return id;
    }

    return Object.keys(itemList).find((key) => itemList[key].includes(id));
  };

  const onDragCancel = () => {
    if (clonedItems) {
      // Reset items to their original state in case items have been
      // Dragged across containers
      setItemList(clonedItems);
    }

    setActiveId(null);
    setClonedItems(null);
  };

  useEffect(() => {
    requestAnimationFrame(() => {
      recentlyMovedToNewContainer.current = false;
    });
  }, [itemList]);

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={collisionDetectionStrategy}
      measuring={{
        droppable: {
          strategy: MeasuringStrategy.Always,
        },
      }}
      onDragStart={({ active }) => {
        setActiveId(toItemId(active.id));
        setClonedItems(itemList);
      }}
      onDragOver={({ active, over }) => {
        const overId = over?.id;

        if (overId == null || active.id in itemList) {
          return;
        }

        const overContainer = findContainerId(toContainerId(overId));
        const activeContainer = findContainerId(toItemId(active.id));

        if (!overContainer || !activeContainer) {
          return;
        }

        if (activeContainer !== overContainer) {
          setItemList((items) => {
            const activeItems = items[activeContainer];
            const overItems = items[overContainer];
            const overIndex = overItems.indexOf(toContainerId(overId));
            const activeIndex = activeItems.indexOf(toItemId(active.id));

            let newIndex: number;

            if (overId in items) {
              newIndex = overItems.length + 1;
            } else {
              const isBelowOverItem =
                over &&
                active.rect.current.translated &&
                active.rect.current.translated.top > over.rect.top + over.rect.height;

              const modifier = isBelowOverItem ? 1 : 0;

              newIndex = overIndex >= 0 ? overIndex + modifier : overItems.length + 1;
            }

            recentlyMovedToNewContainer.current = true;

            return {
              ...items,
              [activeContainer]: items[activeContainer].filter((item) => item !== active.id),
              [overContainer]: [
                ...items[overContainer].slice(0, newIndex),
                items[activeContainer][activeIndex],
                ...items[overContainer].slice(newIndex, items[overContainer].length),
              ],
            };
          });
        }
      }}
      onDragEnd={({ active, over }) => {
        const activeContainer = findContainerId(toItemId(active.id));

        if (!activeContainer) {
          setActiveId(null);
          return;
        }

        const overId = over?.id;

        if (overId == null) {
          setActiveId(null);
          return;
        }

        const overContainer = findContainerId(toContainerId(overId));

        if (overContainer) {
          const activeIndex = itemList[activeContainer].indexOf(toItemId(active.id));
          const overIndex = itemList[overContainer].indexOf(toContainerId(overId));

          if (activeIndex !== overIndex) {
            setItemList((items) => ({
              ...items,
              [overContainer]: arrayMove(items[overContainer], activeIndex, overIndex),
            }));
          }
        }

        setActiveId(null);
      }}
      onDragCancel={onDragCancel}
    >
      <ItemsContext.Provider value={itemList}>{children}</ItemsContext.Provider>
      {createPortal(
        <DragOverlay adjustScale={false} dropAnimation={dropAnimation}>
          {activeId ? renderSortableItemDragOverlay(activeId) : null}
        </DragOverlay>,
        document.body,
      )}
    </DndContext>
  );

  function renderSortableItemDragOverlay(id: ItemId) {
    return (
      <DraggableItem id={id} dragOverlay={true}>
        {renderItem(id)}
      </DraggableItem>
    );
  }
}

function toItemId(id: UniqueIdentifier): ItemId {
  return id.toString();
}

function toContainerId(id: UniqueIdentifier): ContainerId {
  return id.toString();
}

const dropAnimation: DropAnimation = {
  sideEffects: defaultDropAnimationSideEffects({
    styles: {
      active: {
        opacity: '0.5',
      },
    },
  }),
};
