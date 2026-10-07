#[doc = "Register `ALMMIN` reader"]
pub type R = crate::R<AlmminSpec>;
#[doc = "Register `ALMMIN` writer"]
pub type W = crate::W<AlmminSpec>;
#[doc = "Field `ALMMINU` reader - desc ALMMINU"]
pub type AlmminuR = crate::FieldReader;
#[doc = "Field `ALMMINU` writer - desc ALMMINU"]
pub type AlmminuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALMMIND` reader - desc ALMMIND"]
pub type AlmmindR = crate::FieldReader;
#[doc = "Field `ALMMIND` writer - desc ALMMIND"]
pub type AlmmindW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - desc ALMMINU"]
    #[inline(always)]
    pub fn almminu(&self) -> AlmminuR {
        AlmminuR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - desc ALMMIND"]
    #[inline(always)]
    pub fn almmind(&self) -> AlmmindR {
        AlmmindR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ALMMINU"]
    #[inline(always)]
    pub fn almminu(&mut self) -> AlmminuW<'_, AlmminSpec> {
        AlmminuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc ALMMIND"]
    #[inline(always)]
    pub fn almmind(&mut self) -> AlmmindW<'_, AlmminSpec> {
        AlmmindW::new(self, 4)
    }
}
#[doc = "desc ALMMIN\n\nYou can [`read`](crate::Reg::read) this register and get [`almmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlmminSpec;
impl crate::RegisterSpec for AlmminSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`almmin::R`](R) reader structure"]
impl crate::Readable for AlmminSpec {}
#[doc = "`write(|w| ..)` method takes [`almmin::W`](W) writer structure"]
impl crate::Writable for AlmminSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALMMIN to value 0x12"]
impl crate::Resettable for AlmminSpec {
    const RESET_VALUE: u8 = 0x12;
}
