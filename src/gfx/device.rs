#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
use crate::gfx::*;
use crate::swapchain::*;
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use thiserror::Error;
use vulkanalia::vk::KhrSurfaceExtension;

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping physical device (`{}`).", properties.device_name)
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

pub unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    // info!("{:?}", physical_device);
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;
    println!(
        "Checking device: {}",
        instance
            .get_physical_device_properties(physical_device)
            .device_name
    );

    let properties = instance.get_physical_device_properties(physical_device);
    // if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
    //     return Err(anyhow!(SuitabilityError(
    //         "Only Discrete GPUs are supported."
    //     )));
    // }

    let features = instance.get_physical_device_features(physical_device);
    if features.geometry_shader != vk::TRUE {
        return Err(anyhow!(SuitabilityError(
            "Missing geometry shader support."
        )));
    }

    let support = SwapchainSupport::get(instance, data, physical_device)?; //must be after extension check
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
    }

    Ok(())
}

pub unsafe fn check_physical_device_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();
    if crate::DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError(
            "Missing required device extensions."
        )))
    }
}

pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut AppData,
) -> Result<Device> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);
    unique_indices.insert(indices.transfer);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    let layers = if crate::VALIDATION_ENABLED {
        vec![crate::VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    let mut extensions = crate::DEVICE_EXTENSIONS
        .iter()
        .map(|n| n.as_ptr())
        .collect::<Vec<_>>();
    if cfg!(target_os = "macos") && entry.version()? >= crate::PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder();

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);
    data.transfer_queue = device.get_device_queue(indices.transfer, 0);

    Ok(device)
}

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
    pub transfer: u32,
}

impl QueueFamilyIndices {
    pub unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                data.surface,
            )? {
                present = Some(index as u32);
                break;
            }
        }

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let transfer = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::TRANSFER) && !p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if let (Some(graphics), Some(present), Some(transfer)) = (graphics, present, transfer) {
            // println!("gfx: {}\ntransfer: {}", graphics, transfer);
            Ok(Self { graphics, present, transfer })
        } else {
            Err(anyhow!(SuitabilityError(
                        "Missing required queue families."
                        )))
        }
    }
}
