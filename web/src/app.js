import init, { demo_snapshot_json } from "../pkg/car_telemetry.js";
import * as THREE from "https://cdn.jsdelivr.net/npm/three@0.165.0/build/three.module.js";

const format = new Intl.NumberFormat("en-US", { maximumFractionDigits: 1 });

function setText(id, value) {
  document.getElementById(id).textContent = value;
}

function renderMetrics(snapshot) {
  const { current, summary } = snapshot;

  setText("speed", `${format.format(current.speed_mph)} mph`);
  setText("rpm", `${current.rpm.toLocaleString()} rpm`);
  setText("fuel", `${format.format(current.fuel_pct)}%`);
  setText("score", `${summary.efficiency_score}`);
  setText("distance", `${format.format(summary.distance_miles)} mi`);
  setText("average-speed", `${format.format(summary.average_speed_mph)} mph`);
  setText("coolant", `${format.format(current.coolant_f)} F`);
  setText("battery", `${format.format(current.battery_v)} V`);

  const diagnostics = document.getElementById("diagnostics");
  diagnostics.replaceChildren(
    ...snapshot.diagnostics.map((diagnostic) => {
      const item = document.createElement("li");
      item.innerHTML = `<b>${diagnostic.code}</b><span>${diagnostic.severity}</span><strong>${diagnostic.label}</strong>`;
      return item;
    }),
  );

  const timeline = document.getElementById("timeline");
  timeline.replaceChildren(
    ...snapshot.samples.map((sample) => {
      const item = document.createElement("li");
      item.innerHTML = `<span>${sample.second}s</span><strong>${format.format(sample.speed_mph)} mph</strong>`;
      return item;
    }),
  );
}

function normalizeRoute(samples) {
  const latitudes = samples.map((sample) => sample.latitude);
  const longitudes = samples.map((sample) => sample.longitude);
  const minLat = Math.min(...latitudes);
  const maxLat = Math.max(...latitudes);
  const minLon = Math.min(...longitudes);
  const maxLon = Math.max(...longitudes);
  const latSpan = maxLat - minLat || 1;
  const lonSpan = maxLon - minLon || 1;

  return samples.map((sample) => {
    const x = ((sample.longitude - minLon) / lonSpan - 0.5) * 12;
    const z = ((sample.latitude - minLat) / latSpan - 0.5) * -8;
    return new THREE.Vector3(x, sample.speed_mph / 18, z);
  });
}

function buildScene(snapshot) {
  const canvas = document.getElementById("scene");
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.setClearColor(0x111417, 1);

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(46, 1, 0.1, 100);
  camera.position.set(0, 9, 14);
  camera.lookAt(0, 0, 0);

  scene.add(new THREE.HemisphereLight(0xe7fff7, 0x111417, 2.2));

  const grid = new THREE.GridHelper(18, 18, 0x2f6f60, 0x243432);
  grid.position.y = -0.02;
  scene.add(grid);

  const points = normalizeRoute(snapshot.samples);
  const route = new THREE.Line(
    new THREE.BufferGeometry().setFromPoints(points),
    new THREE.LineBasicMaterial({ color: 0x54d2a8 }),
  );
  scene.add(route);

  const pointMaterial = new THREE.MeshStandardMaterial({ color: 0xf8c14a, roughness: 0.42 });
  const currentMaterial = new THREE.MeshStandardMaterial({ color: 0xffffff, emissive: 0x54d2a8 });

  points.forEach((point, index) => {
    const marker = new THREE.Mesh(
      new THREE.SphereGeometry(index === points.length - 1 ? 0.26 : 0.15, 24, 16),
      index === points.length - 1 ? currentMaterial : pointMaterial,
    );
    marker.position.copy(point);
    scene.add(marker);
  });

  const car = new THREE.Group();
  const body = new THREE.Mesh(
    new THREE.BoxGeometry(1.2, 0.38, 0.62),
    new THREE.MeshStandardMaterial({ color: 0x54d2a8, metalness: 0.18, roughness: 0.32 }),
  );
  const cabin = new THREE.Mesh(
    new THREE.BoxGeometry(0.62, 0.34, 0.5),
    new THREE.MeshStandardMaterial({ color: 0xdaf9ef, roughness: 0.18 }),
  );
  cabin.position.set(0.08, 0.34, 0);
  car.add(body, cabin);
  car.position.copy(points.at(-1));
  car.position.y += 0.26;
  scene.add(car);

  function resize() {
    const { clientWidth, clientHeight } = canvas;
    if (canvas.width !== clientWidth || canvas.height !== clientHeight) {
      renderer.setSize(clientWidth, clientHeight, false);
      camera.aspect = clientWidth / Math.max(clientHeight, 1);
      camera.updateProjectionMatrix();
    }
  }

  function animate(time) {
    resize();
    car.rotation.y = Math.sin(time * 0.0007) * 0.18;
    route.rotation.y = Math.sin(time * 0.00015) * 0.08;
    renderer.render(scene, camera);
    requestAnimationFrame(animate);
  }

  animate(0);
}

async function main() {
  await init();
  const snapshot = JSON.parse(demo_snapshot_json());
  renderMetrics(snapshot);
  buildScene(snapshot);
}

main().catch((error) => {
  document.getElementById("connection").textContent = "Demo failed to load";
  console.error(error);
});
