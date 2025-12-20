local MODULE_PATH = [[;.\src\?.lua]]

function isTrainer()
    return getApplication().ExeName:find('cheatengine')
end

if isTrainer() then
    -- 修正 .ct 调用 lua 时的模块 pwd 问题
    if not string.endsWith(package.path, MODULE_PATH) then
        package.path = package.path .. MODULE_PATH
    end
end

local util = require('util')
local app = require('config.app')
local window = require('views.window')
local bootstrap = require('views.bootstrap')
local menu = require('views.menu')
local cover = require('views.cover')
local feature = require('views.feature')

trainer = {
    attached = false,
    platform = nil,
    pid = nil,
}

local function getPlatform()
    local processes = getProcesslist()
    local dlsite_pid, steam_pid = nil, nil

    for pid, name in pairs(processes) do
        if name == app.client.dlsite.name then
            dlsite_pid = pid
        elseif name == app.client.steam.name then
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

local function detachTrainer(both)
    if not trainer.attached then
        return
    end
    -- TODO 双平台客户端同时启动的彩蛋

    trainer.attached = false
    trainer.platform = nil
    trainer.pid = nil

    menu.detach()
    cover.detach()
    feature.detach()
end

local function attachTrainer(platform, pid)
    if trainer.attached and trainer.pid == pid then
        return
    end
    trainer.attached = true
    trainer.platform = platform
    trainer.pid = pid

    openProcess(pid)
    bootstrap.destroy()
    menu.attach()
    cover.attach()
    feature.attach()
    util.playVoice('onj001')
end

local function autoAttach()
    local platform, target_pid, both = getPlatform()

    if not target_pid then
        detachTrainer(both)
    else
        attachTrainer(platform, target_pid)
    end
end

local function create()
    window.draw()
    menu.draw()
    bootstrap.draw()
    cover.draw()
    feature.draw()

    local MainForm = getMainForm()
    local attachTimer = createTimer(MainForm)

    attachTimer.setInterval(1000)
    attachTimer.setOnTimer(autoAttach)

    window.show()
end

create()
