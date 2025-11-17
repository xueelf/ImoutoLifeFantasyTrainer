local function speed(enabled)
    local MainForm = getMainForm()

    if enabled then
        speedhack_setSpeed(3)
    else
        speedhack_setSpeed(1)
    end
    MainForm.cbSpeedhack.Checked = enabled
end

local function victory()
    error('victory')
end

return {
    speed = speed,
    victory = victory,
}
