const API_URL = 'http://localhost:3000/employees';

const btnCargar = document.getElementById('btnCargar');
const cuerpoTabla = document.getElementById('cuerpoTabla');
const mensaje = document.getElementById('mensaje');

const formEmpleado = document.getElementById('formEmpleado');
const inputNombre = document.getElementById('inputNombre');
const inputSalario = document.getElementById('inputSalario');
const mensajeForm = document.getElementById('mensajeForm');

btnCargar.addEventListener('click', cargarEmpleados);
formEmpleado.addEventListener('submit', agregarEmpleado);

async function cargarEmpleados() {
  mensaje.textContent = '';
  cuerpoTabla.innerHTML = '';

  try {
    const respuesta = await fetch(API_URL);
    if (!respuesta.ok) throw new Error('Error del servidor: ' + respuesta.status);

    const empleados = await respuesta.json();

    empleados.forEach(emp => {
      const fila = document.createElement('tr');
      fila.innerHTML = `
        <td>${emp.id}</td>
        <td>${emp.name}</td>
        <td>₡${Number(emp.salary).toLocaleString('es-CR')}</td>
      `;
      cuerpoTabla.appendChild(fila);
    });

  } catch (error) {
    mensaje.textContent = 'No se pudo cargar la lista de empleados: ' + error.message;
  }
}

async function agregarEmpleado(evento) {
  evento.preventDefault();
  mensajeForm.textContent = '';

  const nombre = inputNombre.value.trim();
  const salario = parseFloat(inputSalario.value);

  try {
    const respuesta = await fetch(API_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name: nombre, salary: salario })
    });

    const resultado = await respuesta.json();

    if (!resultado.success) {
      mensajeForm.textContent = resultado.message;
      return;
    }

    formEmpleado.reset();
    cargarEmpleados();

  } catch (error) {
    mensajeForm.textContent = 'No se pudo agregar al empleado: ' + error.message;
  }
}