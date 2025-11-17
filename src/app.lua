local config = require('config')
local util = require('util')
local window = require('views.window')
local cover = require('views.cover')
local feature = require('views.feature')
local menu = require('views.menu')

local font_name = 'Lolita'
local file_icon = 'assets/imouto.ico'

local function create()
    util.loadFont(font_name)

    local Window = window.create()
    local Cover = cover.create(Window)
    local Feature = feature.create(Window)
    local Menu = menu.create(Window)

    Window.Font.setName(font_name)
    Window.Icon.loadFromFile(file_icon)
    Window.centerScreen()

    local function autoAttach()
        local processes = getProcesslist()
        local dlsite_pid = nil
        local steam_pid = nil
        local process_id = getOpenedProcessID()

        for pid, name in pairs(processes) do
            if dlsite_pid and steam_pid then
                break
            elseif name == config.client.dlsite.name then
                dlsite_pid = pid
            elseif name == config.client.steam.name then
                steam_pid = pid
            end
        end
        local target_pid = dlsite_pid or steam_pid

        if not target_pid then
            print('游戏客户端未运行')
        elseif dlsite_pid and steam_pid then
            print('检测到 DLsite 与 Steam 客户端同时运行')
        elseif process_id and process_id ~= target_pid then
            print('正在打开游戏进程...')
            openProcess(target_pid)
            util.playVoice('onj001')
        elseif process_id == target_pid then
            print('游戏进程已打开')
        end
    end
    local timer = createTimer(getMainForm())

    timer.setInterval(1000)
    timer.setOnTimer(autoAttach)
    Window.show()
end

return {
    create = create,
}
