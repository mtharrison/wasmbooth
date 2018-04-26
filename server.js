const Hapi = require('hapi');
const Inert = require('inert');

const server = new Hapi.Server({ port: 4000, host: '0.0.0.0' });

const start = async () => {

    await server.register(Inert);

    server.route({
        method: 'GET',
        path: '/{param*}',
        config: {
            handler: {
                directory: {
                    path: 'public'
                }
            }
        }
    });

    await server.start();
};

start();