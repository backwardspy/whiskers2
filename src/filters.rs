use std::collections::HashMap;

use crate::models::Color;

pub fn modify(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.mod_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.mod_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.mod_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.mod_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}

pub fn add(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.add_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.add_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.add_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.add_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}

pub fn sub(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.sub_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.sub_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.sub_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.sub_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}
