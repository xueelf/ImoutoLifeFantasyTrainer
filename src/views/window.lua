local config = require('config')

local function create()
    local Window = createForm(false)

    Window.Caption = config.window.title
    Window.Width, Window.Height = config.window.width, config.window.height
    Window.OnClose = closeCE

    return Window
end

return {
    create = create,
}
