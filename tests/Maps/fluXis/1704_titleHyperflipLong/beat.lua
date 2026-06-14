---@env storyboard


---@param parent StoryboardElement
function process(parent)
    local texture = parent:param("texture", "fade.png")
    local peak_alpha = parent:param("peak_alpha", 1)
    local fade_in_time = parent:param("fade_in", 40)

    if peak_alpha <= 0 then peak_alpha = 0 end
    if peak_alpha >= 1 then peak_alpha = 1 end
    if fade_in_time <= 0 then fade_in_time = 1 end
    local fade_out_time = parent.endtime - parent.time - fade_in_time


    local left = StoryboardSprite()
    left.time = parent.time
    left.endtime = parent.endtime
    left.x = 0
    left.y = 0
    left.layer = parent.layer
    left.anchor = Anchor("CentreLeft")
    left.origin = Anchor("CentreLeft")
    left.texture = texture
    left:animate("Fade", left.time               , fade_in_time , tostring(0)         , tostring(peak_alpha), "Out")
    left:animate("Fade", left.time + fade_in_time, fade_out_time, tostring(peak_alpha), tostring(0)         , "Out")
    Add(left)

    local right = StoryboardSprite()
    right.time = parent.time
    right.endtime = parent.endtime
    right.x = 0
    right.y = 0
    right.layer = parent.layer
    right.anchor = Anchor("CentreRight")
    right.origin = Anchor("CentreLeft")
    right.texture = texture
    right:animate("Rotate", right.time, 0, tostring(180), tostring(180), "None")
    right:animate("Fade", right.time               , fade_in_time , tostring(0)         , tostring(peak_alpha), "Out")
    right:animate("Fade", right.time + fade_in_time, fade_out_time, tostring(peak_alpha), tostring(0)         , "Out")
    Add(right)
end


DefineParameter("fade_in", "fade in time (ms)", "float")
DefineParameter("peak_alpha", "peak alpha (0-1)", "float")
DefineParameter("texture", "texture", "string")