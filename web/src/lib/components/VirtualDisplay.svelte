<script lang="ts">
  import { T } from "@threlte/core";
  import { OrbitControls } from "@threlte/extras";
  import { onMount } from "svelte";
  import { BufferGeometry, Float32BufferAttribute } from "three";
  import { getDisplay } from "$lib/client";
  import { notify } from "$lib/utils";

  interface Props {
    colors: { r: number; g: number; b: number }[];
  }
  let { colors }: Props = $props();

  let display = $state("");

  let points: number[] = $derived.by(() => {
    let ret = [];

    for (const line of display.split("\n")) {
      const sp = line.split(",");
      if (line.startsWith("#") || sp.length != 6) {
        continue;
      }

      const [x, y, z] = sp.slice(0, 3).map(parseFloat);
      const node = parseInt(sp[3]);
      ret.push([x / 100, y / 100, z / 100, node]);
    }

    return ret
      .sort((a, b) => a[3] - b[3])
      .map((a) => a.slice(0, 3))
      .flat();
  });

  let flatColors: number[] = $derived(
    colors.map((c) => [c.r / 255, c.g / 255, c.b / 255]).flat(),
  );

  const bufGeometry = $derived.by(() => {
    const buf = new BufferGeometry();
    buf.setAttribute("position", new Float32BufferAttribute(points, 3));
    buf.setAttribute("color", new Float32BufferAttribute(flatColors, 3));
    return buf;
  });

  onMount(async () => {
    const { data, error } = await getDisplay();
    if (data) {
      display = data;
    }
    if (error) {
      notify(`${error.error}`, "error");
    }
  });
</script>

<T.PerspectiveCamera makeDefault position={[10, 5, 10]} lookAt.y={0.5}>
  <OrbitControls />
</T.PerspectiveCamera>

<T.DirectionalLight position.y={10} position.z={10} />
<T.AmbientLight intensity={0.3} />

<T.GridHelper args={[10, 10]} />

<T.Points>
  <T is={bufGeometry} />
  <T.PointsMaterial size={0.25} vertexColors />
</T.Points>
