use crate::println;
use raw_cpuid::CpuId;

pub fn init() {
    let cpuid = CpuId::new();

    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!("CPU {}\n", vendor_info);
    }

    if let Some(extended_function_info) = cpuid.get_extended_function_info() {
        if let Some(processor_brand_string) = extended_function_info.processor_brand_string() {
            println!("CPU {}\n", processor_brand_string.trim());
        }
    }

    if let Some(processor_frequency_info) = cpuid.get_processor_frequency_info() {
        let processor_base_frequency = processor_frequency_info.processor_base_frequency();
        println!("CPU {} MHz\n", processor_base_frequency);
    }
}
