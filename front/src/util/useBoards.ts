import { ref, computed, Ref, ComputedRef } from 'vue';
import { Board } from '../types';

interface BoardMap {
  [code: string]: Board;
}

const list = ref([] as Board[]);
const map = computed(() =>
  list.value.reduce(
    (boards: BoardMap, current) => (boards[current.code] = current) && boards,
    {},
  ),
);

export default function useBoards(): {
  map: ComputedRef<BoardMap>;
  list: Ref<Board[]>;
} {
  if (!list.value.length) {
    fetch('/api/boards')
      .then((res) => res.json())
      .then((newList) => (list.value = newList))
      .catch((err) => console.error(err));
  }
  return { map, list };
}
