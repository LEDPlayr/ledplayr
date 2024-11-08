<script lang="ts">
  import type { PerspectiveCamera } from "three";
  import type { Mesh } from "$lib/client";
  import type { CamPos } from "$lib/types";

  import { T, useThrelte } from "@threlte/core";
  import { GLTF, OrbitControls } from "@threlte/extras";
  import { onMount } from "svelte";
  import { BufferGeometry, Float32BufferAttribute, MOUSE } from "three";
  import { OrbitControls as ThreeOrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

  import { getDisplay } from "$lib/client";
  import { notify } from "$lib/utils";

  interface Props {
    colors: { r: number; g: number; b: number }[];
    light: number;
    meshes: Mesh[];
  }
  let { colors, light, meshes }: Props = $props();
  let display = $state("");
  let cam: PerspectiveCamera | undefined = $state();
  let controls: ThreeOrbitControls | undefined = $state();
  const { invalidate } = useThrelte();

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

  export function getCamera(): CamPos | undefined {
    if (cam && controls) {
      return {
        cam_pos_x: cam.position.x,
        cam_pos_y: cam.position.y,
        cam_pos_z: cam.position.z,
        cam_rot_x: cam.rotation.x,
        cam_rot_y: cam.rotation.y,
        cam_rot_z: cam.rotation.z,
        cam_zoom: cam.zoom,
        ctrl_x: controls.target.x,
        ctrl_y: controls.target.y,
        ctrl_z: controls.target.z,
      };
    }
  }

  export function restoreCamera(pos: CamPos) {
    if (cam && controls) {
      cam.position.x = pos.cam_pos_x;
      cam.position.y = pos.cam_pos_y;
      cam.position.z = pos.cam_pos_z;
      cam.rotation.x = pos.cam_rot_x;
      cam.rotation.y = pos.cam_rot_y;
      cam.rotation.z = pos.cam_rot_z;
      cam.zoom = pos.cam_zoom;
      controls.target.x = pos.ctrl_x;
      controls.target.y = pos.ctrl_y;
      controls.target.z = pos.ctrl_z;

      invalidate();
    }
  }
</script>

<T.PerspectiveCamera makeDefault position={[10, 5, 10]} lookAt.y={0.5} bind:ref={cam}>
  <OrbitControls
    bind:ref={controls}
    mouseButtons={{ LEFT: MOUSE.ROTATE, MIDDLE: MOUSE.PAN, RIGHT: MOUSE.DOLLY }} />
</T.PerspectiveCamera>

<T.AmbientLight intensity={light} />

<T.Points>
  <T is={bufGeometry} />
  <T.PointsMaterial size={0.5} vertexColors />
</T.Points>

{#each meshes as m}
  <GLTF
    url={`/api/mesh/${m.name}`}
    scale={[m.scale_x, m.scale_y, m.scale_z]}
    position={[m.pos_x, m.pos_y, m.pos_z]}
    rotation={[m.rot_x, m.rot_y, m.rot_z]} />
{/each}
