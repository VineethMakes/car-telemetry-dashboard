import init, { CarSimulation } from '../pkg/car_telemetry.js';

async function main() {
    await init();
    
    const sim = new CarSimulation();
    
    // Setup Three.js Scene
    const container = document.getElementById('scene-container');
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, container.clientWidth / container.clientHeight, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    
    renderer.setSize(container.clientWidth, container.clientHeight);
    container.appendChild(renderer.domElement);
    
    // Placeholder Car Mesh
    const geometry = new THREE.BoxGeometry(2, 1, 4);
    const material = new THREE.MeshPhongMaterial({ color: 0x00ff00 });
    const carMesh = new THREE.Mesh(geometry, material);
    scene.add(carMesh);
    
    const light = new THREE.DirectionalLight(0xffffff, 1);
    light.position.set(5, 5, 5);
    scene.add(light);
    scene.add(new THREE.AmbientLight(0x404040));
    
    camera.position.set(5, 3, 5);
    camera.lookAt(0, 0, 0);

    let lastTime = performance.now();

    function updateUI(state) {
        document.getElementById('val-speed').textContent = `${state.telemetry.speed.toFixed(1)} km/h`;
        document.getElementById('val-rpm').textContent = state.telemetry.rpm;
        document.getElementById('val-gear').textContent = state.telemetry.gear;
        document.getElementById('val-temp').textContent = `${state.telemetry.engine_temp.toFixed(1)} °C`;
        
        document.getElementById('tire-fl').textContent = `FL: ${state.telemetry.tire_pressure[0].toFixed(1)} PSI`;
        document.getElementById('tire-fr').textContent = `FR: ${state.telemetry.tire_pressure[1].toFixed(1)} PSI`;
        document.getElementById('tire-rl').textContent = `RL: ${state.telemetry.tire_pressure[2].toFixed(1)} PSI`;
        document.getElementById('tire-rr').textContent = `RR: ${state.telemetry.tire_pressure[3].toFixed(1)} PSI`;
        
        const alertsList = document.getElementById('alerts-list');
        alertsList.innerHTML = '';
        if (state.alerts.length === 0) {
            alertsList.innerHTML = '<li class="no-alerts">System Normal</li>';
            material.color.setHex(0x00ff00);
        } else {
            state.alerts.forEach(alert => {
                const li = document.createElement('li');
                li.className = `alert ${alert.severity.toLowerCase()}`;
                li.textContent = alert.message;
                alertsList.appendChild(li);
            });
            material.color.setHex(0xff0000);
        }
    }

    function animate() {
        requestAnimationFrame(animate);
        
        const now = performance.now();
        const dt = (now - lastTime) / 1000;
        lastTime = now;
        
        sim.tick(dt);
        const stateJson = sim.get_state_json();
        const state = JSON.parse(stateJson);
        
        updateUI(state);
        
        // Rotate car slightly based on speed
        carMesh.rotation.y = Math.sin(now * 0.001) * 0.2;
        
        renderer.render(scene, camera);
    }
    
    animate();
    
    window.addEventListener('resize', () => {
        camera.aspect = container.clientWidth / container.clientHeight;
        camera.updateProjectionMatrix();
        renderer.setSize(container.clientWidth, container.clientHeight);
    });
}

main().catch(console.error);
