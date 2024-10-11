<script lang="ts">
  import { T } from "@threlte/core";
  import { OrbitControls } from "@threlte/extras";
  import { onMount } from "svelte";
  import { BufferGeometry, Float32BufferAttribute } from "three";
  import { getDisplay } from "$lib/client";
  import { notify } from "$lib/utils";

  interface Props {
    visible: number[][];
    color: string;
  }
  let { visible, color }: Props = $props();

  let display = $state("");

  let pointColor = $derived.by(() => {
    return hexToRgb(color);
  });

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

  let colors: number[] = $derived.by(() => {
    let ret = [];

    let visibleIdx = 0;
    let [visibleStart, visibleEnd] = [-1, -1];
    if (visible.length > 0) {
      [visibleStart, visibleEnd] = visible[visibleIdx];
    }

    let colorIdx = 0;
    for (let i = 0; i < points.length; i++) {
      let c = [0.1, 0.1, 0.1];

      if (visibleStart <= i && i < visibleEnd) {
        c = pointColor != null ? [pointColor.r, pointColor.g, pointColor.b] : [1, 1, 1];
      }

      if (i == visibleEnd - 1) {
        if (visibleIdx < visible.length - 1) {
          visibleIdx += 1;
          [visibleStart, visibleEnd] = visible[visibleIdx];
        }
      }

      ret.push(c[colorIdx]);
      colorIdx = (colorIdx + 1) % 3;
    }
    return ret;
  });

  const bufGeometry = $derived.by(() => {
    const buf = new BufferGeometry();
    buf.setAttribute("position", new Float32BufferAttribute(points, 3));
    buf.setAttribute("color", new Float32BufferAttribute(colors, 3));
    return buf;
  });

  // https://stackoverflow.com/a/5624139/1965026
  function hexToRgb(hex: string) {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
      ? {
          r: parseInt(result[1], 16) / 255,
          g: parseInt(result[2], 16) / 255,
          b: parseInt(result[3], 16) / 255,
        }
      : null;
  }

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
