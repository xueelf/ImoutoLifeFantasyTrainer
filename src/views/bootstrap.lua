local Bootstrap = createPanel(Window)
local Background = createImage(Bootstrap)
local MainForm = getMainForm()
local animationTimer = createTimer(MainForm)

local function draw()
    Bootstrap.setAlign(alClient)
    Bootstrap.setBevelOuter(bvSpace)
    Bootstrap.setColor(clBlack)

    Background.align = alClient
    Background.center = true
    Background.stretch = true
    Background.proportional = true

    local index = 1
    local images = getFileList('assets/images/logo/')

    local function play()
        index = index + 1

        if index > #images then
            index = 1
        end
        Background.loadImageFromFile(images[index])
    end

    play()
    animationTimer.setInterval(1000)
    animationTimer.setOnTimer(play)
end

local function destroy()
    if not Bootstrap.Visible then
        return
    end
    animationTimer.destroy()
    Bootstrap.setVisible(false)
end

return {
    draw = draw,
    destroy = destroy,
}
