local config = require('config')
local util = require('util')
local window = require('views.window')
local menu = require('views.menu')
local cover = require('views.cover')
local feature = require('views.feature')

tutorial = {
    attached = false,
    platform = nil,
    pid = nil,
}

local function getPlatform()
    local processes = getProcesslist()
    local dlsite_pid, steam_pid = nil, nil

    for pid, name in pairs(processes) do
        if name == config.client.dlsite.name then
            dlsite_pid = pid
        elseif name == config.client.steam.name then
            steam_pid = pid
        end
    end
    local both_running = dlsite_pid and steam_pid

    if both_running then
        return nil, nil, true
    elseif dlsite_pid then
        return 'dlsite', dlsite_pid, false
    elseif steam_pid then
        return 'steam', steam_pid, false
    else
        return nil, nil, false
    end
end

local function detachTutorial(both)
    if not tutorial.attached then
        return
    end
    tutorial.attached = false
    tutorial.platform = nil
    tutorial.pid = nil

    menu.disable()
    cover.unlink()
    feature.hidden()
end

local function attachTutorial(platform, pid)
    if tutorial.attached and tutorial.pid == pid then
        return
    end
    tutorial.attached = true
    tutorial.platform = platform
    tutorial.pid = pid

    openProcess(pid)
    menu.enable()
    cover.link()
    feature.show()
    util.playVoice('onj001')
end

local function autoAttach()
    local platform, target_pid, both = getPlatform()

    if not target_pid then
        detachTutorial(both)
    else
        attachTutorial(platform, target_pid)
    end
end

local function create()
    window.draw()
    menu.draw()
    cover.draw()
    feature.draw()

    local MainForm = getMainForm()
    local timer = createTimer(MainForm)

    timer.setInterval(1000)
    timer.setOnTimer(autoAttach)

    window.show()
end

return {
    create = create,
}
