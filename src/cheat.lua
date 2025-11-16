local function speed(enabled)
    showMessage("speed")
end

local function victory()
    error("victory")
end

return {
    speed = speed,
    victory = victory,
}
