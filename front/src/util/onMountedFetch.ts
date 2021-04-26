import { onMounted, Ref, reactive, toRefs } from 'vue';

export default function onMountedFetch<T>(
  url: string,
  def: T,
): { value: Ref<T>; error: Ref<Error> } {
  const state = reactive({
    value: def,
    error: null,
  }) as { value: T; error: Error };

  onMounted(() => {
    fetch(url)
      .then((res) => res.json())
      .then((json) => (state.value = json))
      .catch((err) => (state.error = err));
  });

  return { ...toRefs(state) };
}
