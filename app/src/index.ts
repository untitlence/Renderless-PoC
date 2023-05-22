const canvas = document.getElementById('canvas') as HTMLCanvasElement

function configure(canvas: HTMLCanvasElement) {
  const { innerWidth, innerHeight } = window

  canvas.width = innerWidth
  canvas.height = innerHeight
}

window.onresize = () => {
  configure(canvas)
}

configure(canvas)

import init, { Backend } from '@crate/backend'
await init()
const backend = Backend.new()

const ctx = canvas.getContext('2d')!

backend.add_player()

const keys = new Set<string>()

document.addEventListener('keydown', ({ key }: KeyboardEvent) => {
  keys.add(key)
})

document.addEventListener('keyup', ({ key }: KeyboardEvent) => {
  keys.delete(key)
})

function frame() {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  const snapshot = backend.render()
  snapshot.world.players.forEach(({ position }) => {
    ctx.fillStyle = 'white'
    ctx.fillRect(position[0] * 20, position[1] * 20, 20, 20)
  })
  keys.forEach((key) => {
    switch (key) {
      case 'w': backend.move_player(0, [0, -1])
        break
      case 'a': backend.move_player(0, [-1, 0])
        break
      case 's': backend.move_player(0, [0, 1])
        break
      case 'd': backend.move_player(0, [1, 0])
        break
    }
  })
  requestAnimationFrame(frame)
}

frame()
