use crate::datapacks::data::{DensityFunction, DensityFunctionImpl, Noise, WorldGen};
use fastnoise2::generator;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

fn next_ident(count: &mut usize) -> Ident {
    let new_ident = format_ident!("_{}", count);
    *count += 1;
    new_ident
}

impl DensityFunction {
    pub fn to_function(&self, ctx: &WorldGen, name: Ident) -> TokenStream {
        let mut counter: usize = 0;
        let mut output = Vec::new();
        self.codegen(ctx, &mut counter, &mut output);

        let func = quote! {
            fn #name() {
                #(#output)*
            }
        };
        func
    }

    pub fn to_noise_tree(&self, ctx: &WorldGen) -> String {
        let noise = fastnoise2::generator::perlin::perlin();
        todo!()
    }
    fn codegen(&self, ctx: &WorldGen, count: &mut usize, output: &mut Vec<TokenStream>) -> Ident {
        match self {
            Self::Id(id) => ctx
                .density_function
                .get(id)
                .unwrap()
                .codegen(ctx, count, output),
            Self::Constant(value) => DensityFunction::codegen_constant(*value, count, output),
            Self::Impl(df) => df.codegen(ctx, count, output),
        }
    }

    fn codegen_constant(value: f64, count: &mut usize, output: &mut Vec<TokenStream>) -> Ident {
        let output_ident = next_ident(count);
        output.push(quote! {
            let #output_ident = #value;
        });
        output_ident
    }
}

impl DensityFunctionImpl {
    fn codegen(&self, ctx: &WorldGen, count: &mut usize, output: &mut Vec<TokenStream>) -> Ident {
        let ident = match self {
            DensityFunctionImpl::Interpolated { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "Interpolated is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::FlatCache { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "FlatCache is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Cache2D { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "Cache2D is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::CacheOnce { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "CacheOnce is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::CacheAllInCell { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "CacheAllInCell is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Abs { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = (#input_ident).abs();
                });
                output_ident
            }
            DensityFunctionImpl::Square { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #input_ident * #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Cube { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #input_ident * #input_ident * #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::HalfNegative { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = {
                        if #input_ident.is_sign_negative() { #input_ident / 2.0 } else { #input_ident }
                    };
                });
                output_ident
            }
            DensityFunctionImpl::QuarterNegative { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = {
                        if #input_ident.is_sign_negative() { #input_ident / 4.0 } else { #input_ident }
                    };
                });
                output_ident
            }
            DensityFunctionImpl::Squeeze { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = {
                        let tmp = #input_ident.clamp(-1.0,1.0);
                        tmp * tmp / 2.0 - tmp * tmp * tmp / 24.0
                    };
                });
                output_ident
            }
            DensityFunctionImpl::Invert { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = 1.0 / #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Add {
                argument1,
                argument2,
            } => {
                let arg1_ident = argument1.codegen(ctx, count, output);
                let arg2_ident = argument2.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #arg1_ident + #arg2_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Mul {
                argument1,
                argument2,
            } => {
                let arg1_ident = argument1.codegen(ctx, count, output);
                let arg2_ident = argument2.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #arg1_ident * #arg2_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Min {
                argument1,
                argument2,
            } => {
                let arg1_ident = argument1.codegen(ctx, count, output);
                let arg2_ident = argument2.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #arg1_ident.min(#arg2_ident);
                });
                output_ident
            }
            DensityFunctionImpl::Max {
                argument1,
                argument2,
            } => {
                let arg1_ident = argument1.codegen(ctx, count, output);
                let arg2_ident = argument2.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #arg1_ident.max(#arg2_ident);
                });
                output_ident
            }
            DensityFunctionImpl::BlendAlpha => {
                output.push(quote! {
                    #[doc = "BlendAlpha is unimplemented!"]
                    let _ = 0.0;
                });
                format_ident!("_")
            }
            DensityFunctionImpl::BlendOffset => {
                output.push(quote! {
                    #[doc = "BlendOffset is unimplemented!"]
                    let _ = 0.0;
                });
                format_ident!("_")
            }
            DensityFunctionImpl::BlendDensity { argument } => {
                let input_ident = argument.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "BlendDensity is unimplemented!"]
                    let #output_ident = #input_ident;
                });
                output_ident
            }
            DensityFunctionImpl::Beardifier => {
                unimplemented!("This should not be referenced in datapack.")
            }
            DensityFunctionImpl::OldBlendedNoise {
                xz_scale,
                y_scale,
                xz_factor,
                y_factor,
                smear_scale_multiplier,
            } => {
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident =
                        blend_noise(#xz_scale, #y_scale, #xz_factor, #smear_scale_multiplier)
                            .at(pos);
                });
                output_ident
            }
            DensityFunctionImpl::Noise {
                noise,
                xz_scale,
                y_scale,
            } => {
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(noise).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    noise, noise
                ));
                output.push(quote! {
                    let #output_ident = noise(#first_octave, &[#(#amplitudes),*])
                        .at(pos * ::bevy_math::DVec3::new(#xz_scale, #y_scale, #xz_scale));
                });
                output_ident
            }
            DensityFunctionImpl::EndIslands => {
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = end_islands().at(pos);
                });
                output_ident
            }
            DensityFunctionImpl::WeirdScaledSampler {
                rarity_value_mapper,
                noise,
                input,
            } => {
                let input_ident = input.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(noise).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    noise, noise
                ));
                let scaling = if rarity_value_mapper == "type_1" {
                    quote! {
                        let scale = match #input_ident {
                            ..-0.5 => 0.75,
                            -0.5..0.0 => 1.0,
                            0.0..0.5 => 1.5,
                            0.5.. => 2.0,
                        };
                    }
                } else {
                    quote! {
                        let scale = match #input_ident {
                            ..-0.75 => 0.5,
                            -0.75..-0.5 => 0.75,
                            -0.5..0.5 => 1,
                            0.5..0.75 => 2.0,
                            0.75.. => 3.0
                        };
                    }
                };
                output.push(quote! {
                    let #output_ident = {
                        #scaling
                        let value = noise(#first_octave, &[#(#amplitudes),*])
                            .at(pos / scale).abs() * scale;
                        value
                    };
                });
                output_ident
            }
            DensityFunctionImpl::ShiftedNoise {
                noise,
                xz_scale,
                y_scale,
                shift_x,
                shift_y,
                shift_z,
            } => {
                let shift_x_ident = shift_x.codegen(ctx, count, output);
                let shift_y_ident = shift_y.codegen(ctx, count, output);
                let shift_z_ident = shift_z.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(noise).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    noise, noise
                ));
                output.push(quote! {
                    let #output_ident = noise(#first_octave, &[#(#amplitudes),*])
                        .at(pos * ::bevy_math::DVec3::new(#xz_scale, #y_scale, #xz_scale)
                            + ::bevy_math::DVec3::new(#shift_x_ident, #shift_y_ident, #shift_z_ident));
                });
                output_ident
            }
            DensityFunctionImpl::RangeChoice {
                input,
                min_inclusive,
                max_exclusive,
                when_in_range,
                when_out_of_range,
            } => {
                let input_ident = input.codegen(ctx, count, output);
                let when_in_range_ident = when_in_range.codegen(ctx, count, output);
                let when_out_of_range_ident = when_out_of_range.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = match #input_ident {
                        #min_inclusive..#max_exclusive => #when_in_range_ident,
                        _ => #when_out_of_range_ident
                    };
                });
                output_ident
            }
            DensityFunctionImpl::ShiftA { argument } => {
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(argument).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    argument, argument
                ));
                output.push(quote! {
                    let #output_ident = noise(#first_octave, &[#(#amplitudes),*])
                        .at(::bevy_math::DVec3::new(pos.x / 4.0, 0.0, pos.z / 4.0)) * 4.0;
                });
                output_ident
            }
            DensityFunctionImpl::ShiftB { argument } => {
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(argument).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    argument, argument
                ));
                output.push(quote! {
                    let #output_ident = noise(#first_octave, &[#(#amplitudes),*])
                        .at(::bevy_math::DVec3::new(pos.z / 4.0, pos.x / 4.0, 0)) * 4.0;
                });
                output_ident
            }
            DensityFunctionImpl::Shift { argument } => {
                let output_ident = next_ident(count);
                let Noise {
                    first_octave,
                    amplitudes,
                } = ctx.noise.get(argument).expect(&format!(
                    "Noise {} should exist in \"data/minecarft/worldgen/noise/{}.json\"",
                    argument, argument
                ));
                output.push(quote! {
                    let #output_ident = noise(#first_octave, &[#(#amplitudes),*])
                        .at(pos / 4.0) * 4.0;
                });
                output_ident
            }
            DensityFunctionImpl::Clamp { input, min, max } => {
                let input_ident = input.codegen(ctx, count, output);
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = (#input_ident).clamp(#min, #max);
                });
                output_ident
            }
            DensityFunctionImpl::Spline { spline } => {
                let output_ident = next_ident(count);
                output.push(quote! {
                    #[doc = "Spline is unimplemented."]
                    let #output_ident = 0.0;
                });
                output_ident
            }
            DensityFunctionImpl::Constant { argument } => {
                DensityFunction::codegen_constant(*argument, count, output)
            }
            DensityFunctionImpl::YClampedGradient {
                from_y,
                to_y,
                from_value,
                to_value,
            } => {
                let output_ident = next_ident(count);
                output.push(quote! {
                    let #output_ident = #from_value
                        + (#to_value - #from_value)
                            * ((pos.y.clamp(#from_y, #to_y) - #from_y) / (#to_y - #from_y));
                });
                output_ident
            }
            DensityFunctionImpl::FindTopSurface {
                density,
                upper_bound,
                lower_bound,
                cell_height,
            } => todo!(),
        };
        ident
    }
}
