const { saludar } = require('./index');

try {

    const result = saludar('bat briones');
    console.log(result);

} catch (error) {
    console.error('Error al llamar a la función saludar:', error);
}