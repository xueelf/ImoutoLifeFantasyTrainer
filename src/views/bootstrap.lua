local Bootstrap = createPanel(Window)
local Background = createImage(Bootstrap)
local MainForm = getMainForm()
local animationTimer = createTimer(MainForm)

local BOOTSTRAP_LOGO = 'assets/images/logo/'

local function draw()
    Bootstrap.setAlign(alClient)
    Bootstrap.setBevelOuter(bvSpace)
    Bootstrap.setColor(clBlack)

    Background.align = alClient
    Background.center = true
    Background.stretch = true
    Background.proportional = true

    local images = getFileList(BOOTSTRAP_LOGO)
    local index = 1
    local timer_tick_count = 0
    local timer_tick_max = 30

    local function linkStart()
        index = index + 1
        timer_tick_count = timer_tick_count + 1

        if timer_tick_count >= timer_tick_max then
            animationTimer.destroy()
            showMessage('哥哥喜欢放置 Play 是么？')
            closeCE()
        end
        if index > #images then
            index = 1
        end
        Background.loadImageFromFile(images[index])
    end

    linkStart()
    animationTimer.setInterval(1000)
    animationTimer.setOnTimer(linkStart)
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
