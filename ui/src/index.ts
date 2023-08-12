import { Game } from 'game/dimensions/game'
import run from 'game/operations/run'

const game = Game.build('dimensions', 400, 400)
run.setup(game)
run.draw(game)
