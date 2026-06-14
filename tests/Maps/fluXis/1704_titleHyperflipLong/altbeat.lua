---@env storyboard

---@param parent StoryboardElement
function process(parent)
    local beat_interval = parent:param("beat_interval", 25)
    local texture = parent:param("texture", "fade.png")
    local peak_alpha = parent:param("peak_alpha", 1)
    local fade_in_time = parent:param("fade_in", 12)
    local fade_out_time = parent:param("fade_out", 123)
    local side = parent:param("start_side", 0)
    
    if beat_interval <= 0 then beat_interval = 1 end
    if peak_alpha <= 0 then peak_alpha = 0 end
    if peak_alpha >= 1 then peak_alpha = 1 end
    if fade_in_time <= 0 then fade_in_time = 1 end
    if fade_out_time <= 0 then fade_out_time = 1 end

    for t=parent.time, parent.endtime, beat_interval do
        local beat = StoryboardSprite()
        beat.time = t
        beat.endtime = t + fade_in_time + fade_out_time
        beat.x = 0
        beat.y = 0
        beat.layer = parent.layer
        beat.origin = Anchor("CentreLeft")
        beat.texture = texture

        if side%2 == 0 then
            beat.anchor = Anchor("CentreLeft")
        else
            beat.anchor = Anchor("CentreRight")
            beat:animate("Rotate", beat.time, 0, tostring(180), tostring(180), "None")
        end

        beat:animate("Fade", beat.time               , fade_in_time , tostring(0)         , tostring(peak_alpha), "None")
        beat:animate("Fade", beat.time + fade_in_time, fade_out_time, tostring(peak_alpha), tostring(0)         , "None")

        Add(beat)

        side = side + 1
    end
end

DefineParameter("beat_interval", "beat interval (ms)", "float")
DefineParameter("fade_in", "fade in time (ms)", "float")
DefineParameter("fade_out", "fade out time (ms)", "float")
DefineParameter("peak_alpha", "peak alpha (0-1)", "float")
DefineParameter("texture", "texture", "string")
DefineParameter("start_side", "start side (0:L, 1:R)", "int")

