import init, { run, InputRunner } from '../../core/pkg/core.js'

await init()

run(new InputRunner('Nickname', 'Description'))
